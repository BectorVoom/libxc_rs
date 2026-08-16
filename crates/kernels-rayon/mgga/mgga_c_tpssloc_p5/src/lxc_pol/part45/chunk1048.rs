//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 45 (v4rho2sigma2_1) CSE chunk 1048/1056 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part45_v4rho2sigma2_1_chunk1048(t22574: f64, t31299: f64, t32193: f64, t22480: f64, t7042: f64, t23929: f64, t8526: f64, t1307: f64, t26558: f64, t31775: f64, t22607: f64, t8641: f64) -> (f64, f64, f64, f64, f64) {
    let t115942 = 6.0_f64 * t22574 * t32193 * t31299;
    let t115946 = 2.0_f64 * t7042 * t22480;
    let t115948 = 4.0_f64 * t8526 * t23929;
    let t115959 = 12.0_f64 * t22574 * t26558 * t31775 * t1307;
    let t115965 = t22607 * t8641;
    (t115942, t115946, t115948, t115959, t115965)
}
