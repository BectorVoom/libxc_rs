//! MGGA_C_KCIS kxc pol — kxc_pol part 4 (v3rho3_1) CSE chunk 771/1420 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_kxc_pol_part4_v3rho3_1_chunk771(t304: f64, t346: f64, t1045: f64, t1728: f64, t1670: f64, t934: f64, t1724: f64, t932: f64, t2943: f64, t4625: f64, t2919: f64, t3088: f64, t4612: f64, t4615: f64, t4618: f64, t4623: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t4642 = t304 * t346;
    let t4643 = t1728 * t1045;
    let t4644 = t4642 * t4643;
    let t4647 = t1670 * t934;
    let t4654 = t932 * t1724;
    let t4657 = t2943 * t1670;
    let t4658 = t4657 * t934;
    let t4660 = t932 * t4625;
    let t4667 = -0.991e-2_f64 * t4658 + 0.1982e-1_f64 * t4660 + t3088 + 0.13758333333333333333e-2_f64 * t2919 + 0.13758333333333333333e-2_f64 * t4612 - 0.27516666666666666667e-2_f64 * t4615 + 0.8255e-2_f64 * t4618 - 0.8255e-2_f64 * t4623;
    (t4642, t4643, t4644, t4647, t4654, t4657, t4658, t4660, t4667)
}
