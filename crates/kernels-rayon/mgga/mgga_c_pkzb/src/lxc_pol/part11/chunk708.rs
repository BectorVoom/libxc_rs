//! MGGA_C_PKZB lxc pol — lxc_pol part 11 (v4rho4_3) CSE chunk 708/1340 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_pkzb_lxc_pol_part11_v4rho4_3_chunk708(t110: f64, t1564: f64, t1569: f64, t1572: f64, t1582: f64, t1590: f64, t1608: f64, t1615: f64, t1618: f64, t1622: f64, t204: f64, t465: f64, t49: f64, t4902: f64, t5019: f64, t5022: f64, t5025: f64, t5028: f64, t5029: f64, t5040: f64, t5044: f64, t5048: f64, t5052: f64, t5056: f64, t5066: f64, t5069: f64, t5073: f64, t527: f64, t542: f64) -> f64 {
    let t5074 = 0.16562821945185185185e-2_f64 * t49 * t4902 * t110 - t5019 + t5022 - t5025 - t5028 + 0.32530743900905219526e-1_f64 * t204 * t5029 * t1615 + 0.10274e0_f64 * t204 * t465 * t1569 * t1572 - t5040 - 0.51369999999999999999e-1_f64 * t204 * t1564 * t1582 - 0.16522625736956710527e1_f64 * t204 * t5044 * t1590 + 0.68493333333333333332e-1_f64 * t204 * t5048 * t527 - 0.48159733137676571078e0_f64 * t204 * t5052 * t1622 + 0.21687162600603479684e-1_f64 * t204 * t5056 * t542 - 0.16265371950452609763e-1_f64 * t204 * t1608 * t1618 - t5066 + t5069 + t5073;
    t5074
}
