//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 19 (v4rho4_0) CSE chunk 1130/1497 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1130(t39514: f64, t677: f64, t9919: f64, t3684: f64, t2393: f64, t2535: f64, t12110: f64, t9882: f64, t12466: f64, t3719: f64, t3918: f64, t39483: f64, t39490: f64, t39492: f64, t39496: f64, t39499: f64, t39502: f64, t39505: f64, t39508: f64, t39511: f64, t39513: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t39515 = 0.86748650402413918736e-1_f64 * t39514;
    let t39516 = t677 * t9919;
    let t39518 = 0.1301229756036208781e0_f64 * t3684 * t39516;
    let t39519 = t2393 * t2535;
    let t39521 = 0.43374325201206959368e-1_f64 * t3684 * t39519;
    let t39522 = t12110 * t9882;
    let t39523 = 0.1301229756036208781e0_f64 * t39522;
    let t39524 = 18.0_f64 * t12466 * t3719 * t3918 + t39483 - t39490 + t39492 - t39496 + t39499 + t39502 - t39505 - t39508 + t39511 + t39513 - t39515 + t39518 - t39521 - t39523;
    (t39515, t39516, t39518, t39519, t39521, t39523, t39524)
}
