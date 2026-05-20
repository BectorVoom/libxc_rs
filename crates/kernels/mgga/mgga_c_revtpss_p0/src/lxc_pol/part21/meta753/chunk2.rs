//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 2637/3259 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2637<F: Float>(t1872: F, t3924: F, t9816: F, t9818: F, t13848: F, t47274: F, t9956: F, t13878: F, t9765: F, t13869: F, t3989: F, t2661: F, t5608: F, t9840: F, t9934: F) -> (F, F, F, F, F) {
    let t48494 = t9816 * t9818 * t1872 * t3924;
    let t48498 = t9816 * t47274 * t13848 * t9956;
    let t48508 = t9765 * t13878;
    let t48509 = F::cast_from(0.8131200449485652516e-2_f64) * t48508;
    let t48510 = t3989 * t13869;
    let t48514 = t2661 * t9934 * t5608 * t9840;
    (t48494, t48498, t48509, t48510, t48514)
}
