//! MGGA_C_KCIS lxc pol — lxc_pol part 24 (v4rho3sigma_6) CSE chunk 691/1171 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part24_v4rho3sigma_6_chunk691<F: Float>(t138: F, t86: F, t8955: F, t124: F, t2394: F, t2479: F, t66: F, t717: F, t740: F, t113: F, t2438: F, t2437: F, t95: F, t89: F, t160: F, t4858: F, t4865: F, t4881: F, t822: F) -> (F, F, F, F, F, F, F, F, F) {
    let t8957 = t86 * t8955 * t138;
    let t8959 = t2394 * t124;
    let t8961 = t86 * t8959 * t138;
    let t8963 = t66 * t2479;
    let t8965 = t86 * t8963 * t138;
    let t8969 = t740 * t717;
    let t8972 = t113 * t2438;
    let t8978 = 1.0 / t2437 / t95;
    let t8979 = t89 * t8978;
    let t8996 = 0.35867157975189532869e-1 * t160 - 0.13661666666666666667e-1 * t4858 + 0.38744444444444444446e-2 * t4865 - 0.15538616723388920628e-3 * t822 + 0.18204739583333333333e-4 * t4881;
    (t8957, t8959, t8961, t8963, t8965, t8969, t8972, t8979, t8996)
}
