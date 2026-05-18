//! MGGA_C_REVTPSS lxc pol — lxc_pol part 52 (v4rho2sigma2_7) CSE chunk 1146/1292 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part52_v4rho2sigma2_7_chunk1146<F: Float>(t25331: F, t32481: F, t119842: F, t2453: F, t25301: F, t32477: F, t121803: F, t1955: F, t119869: F, t32478: F, t2470: F, t32470: F) -> (F, F, F, F, F, F, F) {
    let t121851 = F::new(0.34270468708064099208e-1) * t32481 * t25331;
    let t121855 = F::new(0.98339826130601561944e-2) * t119842;
    let t121869 = F::new(0.3427046870806409921e-2) * t2453 * t32477 * t25301;
    let t121870 = t1955 * t121803;
    let t121879 = F::new(0.35702867204846465857e-4) * t119869;
    let t121881 = F::new(0.19274729307122665472e-1) * t32478 * t25331;
    let t121884 = t32470 * t2470;
    (t121851, t121855, t121869, t121870, t121879, t121881, t121884)
}
