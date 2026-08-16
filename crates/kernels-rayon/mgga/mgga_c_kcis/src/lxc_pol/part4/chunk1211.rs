//! MGGA_C_KCIS lxc pol — lxc_pol part 4 (v3rho3_1) CSE chunk 1211/1420 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part4_v3rho3_1_chunk1211(t11061: f64, t1846: f64, t1251: f64, t2470: f64, t992: f64, t5315: f64, t3611: f64, t5330: f64, t5329: f64, t11000: f64, t1851: f64, t3532: f64) -> (f64, f64, f64, f64) {
    let t15548 = t11061 * t1846;
    let t15549 = t1251 * t15548;
    let t15553 = t2470 * t992;
    let t15554 = t15553 * t5315;
    let t15555 = t1251 * t15554;
    let t15557 = t5330 * t3611;
    let t15558 = t5329 * t15557;
    let t15562 = t11000 * t1851 * t3532;
    (t15549, t15555, t15558, t15562)
}
