//! MGGA_C_KCIS kxc pol — kxc_pol part 5 (v3rho3_2) CSE chunk 928/1419 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_kxc_pol_part5_v3rho3_2_chunk928<F: Float>(t138: F, t86: F, t8963: F, t717: F, t740: F, t113: F, t2438: F, t2437: F, t95: F, t89: F, t160: F, t4858: F, t4865: F, t4881: F, t822: F) -> (F, F, F, F, F) {
    let t8965 = t86 * t8963 * t138;
    let t8969 = t740 * t717;
    let t8972 = t113 * t2438;
    let t8978 = F::new(1.0) / t2437 / t95;
    let t8979 = t89 * t8978;
    let t8996 = F::cast_from(0.35867157975189532869e-1_f64) * t160 - F::cast_from(0.13661666666666666667e-1_f64) * t4858 + F::cast_from(0.38744444444444444446e-2_f64) * t4865 - F::cast_from(0.15538616723388920628e-3_f64) * t822 + F::cast_from(0.18204739583333333333e-4_f64) * t4881;
    (t8965, t8969, t8972, t8979, t8996)
}
