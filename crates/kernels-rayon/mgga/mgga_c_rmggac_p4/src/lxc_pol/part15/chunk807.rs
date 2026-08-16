//! MGGA_C_RMGGAC lxc pol — lxc_pol part 15 (v4rho3sigma_6) CSE chunk 807/1110 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part15_v4rho3sigma_6_chunk807(t1965: f64, t9085: f64, t1969: f64, t2305: f64, t35654: f64, t16502: f64, t8516: f64, t5016: f64, t9000: f64, t1605: f64, t1986: f64, t8817: f64, t942: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t39392 = t9085 * t1965;
    let t39393 = t39392 * t1969;
    let t39405 = t35654 * t2305;
    let t39406 = 0.19863479950205658386e-4_f64 * t39405;
    let t39437 = t8516 * t16502;
    let t39451 = t5016 * t9000;
    let t39452 = 0.15965655602485078085e0_f64 * t39451;
    let t39490 = t1986 * t1605;
    let t39506 = 0.4726e1_f64 * t942 * t8817;
    (t39393, t39406, t39437, t39452, t39490, t39506)
}
