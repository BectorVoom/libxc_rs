//! MGGA_C_KCIS lxc pol — lxc_pol part 26 (v4rho3sigma_8) CSE chunk 816/1397 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part26_v4rho3sigma_8_chunk816<F: Float>(t1370: F, t4455: F, t1607: F, t3978: F, t1606: F, t4354: F, t597: F, t592: F, t11407: F, t11481: F, t1562: F, t4357: F, t600: F) -> (F, F, F, F, F, F, F, F) {
    let t12605 = t1370 * t4455;
    let t12617 = t3978 * t1607;
    let t12650 = t1606 * t1606;
    let t12651 = F::cast_from(1.0_f64) / t12650;
    let t12688 = F::cast_from(1.0_f64) / t4354 / t597;
    let t12689 = t592 * t12688;
    let t12717 = F::cast_from(0.16068111111111111111e1_f64) * t11407;
    let t12718 = F::cast_from(0.46308888888888888888e0_f64) * t11481;
    let t12729 = F::cast_from(1.0_f64) / t4354 / t1562;
    let t12730 = t592 * t12729;
    let t12732 = F::cast_from(1.0_f64) / t4357 / t600;
    (t12605, t12617, t12651, t12689, t12717, t12718, t12730, t12732)
}
