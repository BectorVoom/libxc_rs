//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 21 (v4rho4_2) CSE chunk 900/3221 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk900(t3237: f64, t3238: f64, t3245: f64, t3250: f64, t3254: f64, t423: f64, t1094: f64, t1098: f64) -> (f64, f64, f64) {
    let t3256 = t3237 - 0.11872222222222222222e-1_f64 * t3238 - 0.11872222222222222222e-1_f64 * t3245 + 0.35616666666666666666e-1_f64 * t3250 + 0.17808333333333333333e-1_f64 * t3254;
    let t3258 = 0.621814e-1_f64 * t3256 * t423;
    let t3259 = t1094 * t1098;
    (t3256, t3258, t3259)
}
