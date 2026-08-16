//! MGGA_C_KCIS lxc pol — lxc_pol part 5 (v3rho3_2) CSE chunk 1034/1419 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part5_v3rho3_2_chunk1034(t15516: f64, t3514: f64, t421: f64, t4951: f64, t3490: f64, t5299: f64, t11061: f64, t1846: f64, t1251: f64, t2470: f64, t992: f64, t5315: f64) -> (f64, f64, f64, f64, f64) {
    let t15518 = t3514 * t15516 / 864.0_f64;
    let t15534 = t4951 * t421;
    let t15547 = t3490 * t5299 / 324.0_f64;
    let t15548 = t11061 * t1846;
    let t15549 = t1251 * t15548;
    let t15553 = t2470 * t992;
    let t15554 = t15553 * t5315;
    (t15518, t15534, t15547, t15549, t15554)
}
