//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 3178/3259 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk3178(t43865: f64, t43883: f64, t43888: f64, t43890: f64, t43892: f64, t43894: f64, t43896: f64, t58186: f64, t58189: f64, t58192: f64, t58195: f64, t58198: f64) -> f64 {
    let t58572 = -0.26574814814814814816e0_f64 * t43865 + 0.39862222222222222222e0_f64 * t43883 - 0.93011851851851851855e0_f64 * t43888 + 0.39862222222222222224e0_f64 * t43890 + 0.79724444444444444447e0_f64 * t43892 - 0.59793333333333333333e0_f64 * t43894 - 0.99655555555555555557e-1_f64 * t43896 - 0.65725333333333333332e0_f64 * t58186 - 0.82156666666666666668e-1_f64 * t58189 - 0.82156666666666666668e-1_f64 * t58192 - 0.49294e0_f64 * t58195 - 0.27385555555555555556e-1_f64 * t58198;
    t58572
}
