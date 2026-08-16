//! MGGA_C_KCIS lxc pol — lxc_pol part 5 (v3rho3_2) CSE chunk 660/1419 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part5_v3rho3_2_chunk660<F: Float>(t304: F, t346: F, t1045: F, t1728: F, t1670: F, t934: F, t1724: F, t932: F, t2943: F, t4625: F, t2919: F, t3088: F, t4612: F, t4615: F, t4618: F, t4623: F) -> (F, F, F, F, F, F, F, F, F) {
    let t4642 = t304 * t346;
    let t4643 = t1728 * t1045;
    let t4644 = t4642 * t4643;
    let t4647 = t1670 * t934;
    let t4654 = t932 * t1724;
    let t4657 = t2943 * t1670;
    let t4658 = t4657 * t934;
    let t4660 = t932 * t4625;
    let t4667 = -F::cast_from(0.991e-2_f64) * t4658 + F::cast_from(0.1982e-1_f64) * t4660 + t3088 + F::cast_from(0.13758333333333333333e-2_f64) * t2919 + F::cast_from(0.13758333333333333333e-2_f64) * t4612 - F::cast_from(0.27516666666666666667e-2_f64) * t4615 + F::cast_from(0.8255e-2_f64) * t4618 - F::cast_from(0.8255e-2_f64) * t4623;
    (t4642, t4643, t4644, t4647, t4654, t4657, t4658, t4660, t4667)
}
