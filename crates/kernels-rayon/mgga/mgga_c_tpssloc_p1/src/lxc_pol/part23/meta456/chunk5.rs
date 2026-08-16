//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 23 (v4rho4_4) CSE chunk 1324/1527 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1324(t76359: f64, t76371: f64, t225: f64, t13222: f64, t13228: f64, t1512: f64, t20953: f64, t237: f64, t249: f64, t4167: f64, t4178: f64, t59259: f64, t59263: f64, t59276: f64, t59288: f64, t67872: f64, t68148: f64, t68195: f64, t68197: f64, t68199: f64, t68201: f64, t76250: f64) -> (f64, f64, f64) {
    let t76372 = t76359 + t76371;
    let t76373 = t76372 * t225;
    let t76394 = t76373 * t237 * t249 / 3072.0_f64 - 7.0_f64 / 4.0_f64 * t68148 - 119.0_f64 / 288.0_f64 * t59259 - 119.0_f64 / 576.0_f64 * t59263 - t4167 * t20953 / 768.0_f64 - t67872 * t1512 / 768.0_f64 - 119.0_f64 / 2304.0_f64 * t59276 + 119.0_f64 / 2304.0_f64 * t59288 + 35.0_f64 / 48.0_f64 * t68195 - 35.0_f64 / 96.0_f64 * t68197 + 7.0_f64 / 96.0_f64 * t68199 + 7.0_f64 / 96.0_f64 * t68201 - t4178 * t13222 * t13228 * t76250 / 32.0_f64;
    (t76372, t76373, t76394)
}
