//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 28 (v4rho3sigma_4) CSE chunk 1484/2041 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1484(t1216: f64, t4733: f64, t3578: f64, t1653: f64, t3494: f64, t1090: f64, t5012: f64, t3490: f64, t4993: f64, t248: f64, t3521: f64, t1227: f64) -> (f64, f64, f64, f64, f64) {
    let t15469 = t4733 * t1216;
    let t15470 = t3578 * t15469;
    let t15473 = t1653 * t3494;
    let t15474 = t3578 * t15473;
    let t15477 = t5012 * t1090;
    let t15478 = t3578 * t15477;
    let t15484 = t3490 * t4993 / 3456.0_f64;
    let t15486 = t248 * t3521 * t4733;
    let t15488 = t1227 * t15486 / 3456.0_f64;
    (t15470, t15474, t15478, t15484, t15488)
}
