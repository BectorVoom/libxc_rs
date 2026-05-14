//! GGA_C_ACGGAP lxc pol — lxc_pol part 5 (v4rho4_2) CSE chunk 977/1191 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part5_v4rho4_2_chunk977<F: Float>(t43: F, t14892: F, t192: F, t5506: F, t14898: F, t14900: F, t14902: F, t14904: F, t11683: F, t11696: F, t234: F, t34: F, t821: F, t12161: F, t1281: F, t15072: F, t1690: F, t1694: F, t2868: F, t2898: F, t35: F, t4070: F, t5455: F, t5481: F, t5486: F, t595: F, t817: F, t818: F, t824: F, zeta_threshold: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t44 = t43 <= zeta_threshold;
    let t19441 = 0.65061487801810439052e-1 * t14892;
    let t19444 = t192 * t5506;
    let t19451 = 0.36622894612013090108e-3 * t14898;
    let t19452 = 0.97661052298701573622e-3 * t14900;
    let t19453 = 0.2077903092681775651e3 * t14902;
    let t19454 = 0.46785788981077169656e1 * t14904;
    let t19455 = 0.70178683471615754484e1 * t11683;
    let t19456 = 12.0 * t11696;
    let t19461 = t234 * t34 * t821;
    let t19482 = piecewise3(t44, 0.0, -56.0 / 81.0 * t12161 * t1690 * t818 + 64.0 / 27.0 * t4070 * t19461 + 8.0 / 27.0 * t5481 * t824 - 16.0 / 9.0 * t817 * t35 * t595 - 8.0 / 9.0 * t1281 * t821 + 8.0 / 3.0 * t1281 * t2868 + 8.0 / 27.0 * t2898 * t1694 * t818 - 4.0 / 9.0 * t817 * t5455 * t234 - 2.0 / 9.0 * t5486 * t824 + t15072);
    (t19441, t19444, t19451, t19452, t19453, t19454, t19455, t19456, t19461, t19482)
}
