//! MGGA_C_REVTPSS lxc pol — lxc_pol part 52 (v4rho2sigma2_7) CSE chunk 1144/1292 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part52_v4rho2sigma2_7_chunk1144<F: Float>(t121809: F, t25413: F, t121804: F, t7060: F, t786: F, t103181: F, t32470: F, t119982: F, t25296: F, t32481: F, t25301: F, t25304: F, t32477: F) -> (F, F, F, F, F, F) {
    let t121810 = t121809 * t25413;
    let t121815 = t786 * t121804 * t7060;
    let t121817 = t32470 * t103181;
    let t121818 = t119982 * t121817;
    let t121820 = t32481 * t25296;
    let t121825 = F::new(0.45699670022203476294e-2) * t25304 * t32477 * t25301;
    (t121810, t121815, t121817, t121818, t121820, t121825)
}
