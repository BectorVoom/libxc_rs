//! MGGA_C_REVTPSS lxc pol — lxc_pol part 51 (v4rho2sigma2_6) CSE chunk 1039/1200 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part51_v4rho2sigma2_6_chunk1039<F: Float>(t124: F, t800: F, t815: F, t886: F, t32474: F, t51076: F, t7076: F, t2453: F, t8648: F, t25304: F, t119971: F, t32469: F) -> (F, F, F, F, F) {
    let t120106 = t815 * t800 * t124 * t886;
    let t120107 = t32474 * t120106;
    let t120110 = t7076 * t51076;
    let t120111 = t2453 * t8648 * t120110;
    let t120112 = F::cast_from(0.3718732920905101082e-5_f64) * t120111;
    let t120114 = t25304 * t8648 * t120110;
    let t120115 = F::cast_from(0.19835721400107809171e-4_f64) * t120114;
    let t120117 = t119971 * t8648 * t120110;
    let t120118 = F::cast_from(0.23511941766261123138e-4_f64) * t120117;
    let t120119 = t32469 * t120106;
    (t120107, t120112, t120115, t120118, t120119)
}
