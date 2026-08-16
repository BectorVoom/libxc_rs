//! MGGA_C_R2SCAN lxc pol — lxc_pol part 14 (v4rho3sigma_4) CSE chunk 1238/1276 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_r2scan_lxc_pol_part14_v4rho3sigma_4_chunk1238(t11506: f64, t39015: f64, t2867: f64, t3275: f64, t38739: f64, t11002: f64, t1115: f64, t2847: f64, t3269: f64, t39197: f64, t39198: f64, t3262: f64, t3472: f64, t40635: f64) -> (f64, f64, f64, f64, f64) {
    let t41811 = 3.0_f64 / 2.0_f64 * t11506 * t39015;
    let t41814 = t3275 * t38739 * t2867 / 4.0_f64;
    let t41816 = t11002 * t1115 * t2847;
    let t41818 = 5.0_f64 / 8.0_f64 * t3269 * t41816;
    let t41821 = 15.0_f64 / 4.0_f64 * t39197 * t1115 * t39198;
    let t41824 = 15.0_f64 / 8.0_f64 * t3262 * t3472 * t40635;
    (t41811, t41814, t41818, t41821, t41824)
}
