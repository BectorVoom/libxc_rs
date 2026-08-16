//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 33 (v4rho3sigma_9) CSE chunk 1350/1415 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part33_v4rho3sigma_9_chunk1350(t1622: f64, t1935: f64, t21444: f64, t21532: f64, t21538: f64, t21566: f64, t21597: f64, t23419: f64, t25577: f64, t28526: f64, t343: f64, t5869: f64, t6717: f64, t6734: f64, t6755: f64, t7578: f64, t83215: f64, t99590: f64, t99624: f64, t99631: f64, t99731: f64) -> f64 {
    let t106267 = t99590 / 768.0_f64 - t99624 / 144.0_f64 + t25577 * t5869 / 512.0_f64 + 0.30279567070605293142e-3_f64 * t99631 + t6755 * t21597 / 1536.0_f64 + t99731 * t1622 / 768.0_f64 - 0.10093189023535097714e-3_f64 * t1935 * t21444 * t343 * t6734 - 0.30279567070605293142e-3_f64 * t28526 * t7578 - t6717 * t21538 / 36.0_f64 - t83215 * t21532 / 768.0_f64 + t23419 * t21566 / 768.0_f64;
    t106267
}
