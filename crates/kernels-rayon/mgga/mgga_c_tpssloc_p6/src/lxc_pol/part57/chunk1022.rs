//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 57 (v4rho2sigma2_13) CSE chunk 1022/1049 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part57_v4rho2sigma2_13_chunk1022(t28025: f64, t7042: f64, t28827: f64, t8607: f64, t33336: f64, t7685: f64, t28821: f64, t8644: f64, t128516: f64, t128523: f64, t128535: f64, t128537: f64, t128539: f64, t1976: f64, t2075: f64, t24999: f64, t27188: f64, t27996: f64, t28951: f64, t28952: f64, t29214: f64, t29219: f64, t29243: f64, t33085: f64, t6517: f64, t652: f64, t7472: f64, t7802: f64, t8450: f64) -> f64 {
    let t128543 = 2.0_f64 * t7042 * t28025;
    let t128549 = 6.0_f64 * t8607 * t28827;
    let t128551 = 2.0_f64 * t7685 * t33336;
    let t128552 = t28821 * t8644;
    let t128553 = -2.0_f64 * t1976 * t28951 * t652 - 2.0_f64 * t2075 * t27996 - 4.0_f64 * t24999 * t7802 - 4.0_f64 * t27188 * t7472 - 2.0_f64 * t28952 * t6517 - 2.0_f64 * t29214 * t6517 - 4.0_f64 * t29219 * t6517 + 2.0_f64 * t29243 * t8450 - 4.0_f64 * t33085 * t7802 - t128516 - t128523 - t128535 - t128537 - t128539 - t128543 + t128549 + t128551 - t128552;
    t128553
}
