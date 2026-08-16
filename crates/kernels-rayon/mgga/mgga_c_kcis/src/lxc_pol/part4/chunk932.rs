//! MGGA_C_KCIS lxc pol — lxc_pol part 4 (v3rho3_1) CSE chunk 932/1420 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part4_v3rho3_1_chunk932(t138: f64, t86: f64, t8963: f64, t717: f64, t740: f64, t113: f64, t2438: f64, t2437: f64, t95: f64, t89: f64, t160: f64, t4858: f64, t4865: f64, t4881: f64, t822: f64) -> (f64, f64, f64, f64, f64) {
    let t8965 = t86 * t8963 * t138;
    let t8969 = t740 * t717;
    let t8972 = t113 * t2438;
    let t8978 = 1.0_f64 / t2437 / t95;
    let t8979 = t89 * t8978;
    let t8996 = 0.35867157975189532869e-1_f64 * t160 - 0.13661666666666666667e-1_f64 * t4858 + 0.38744444444444444446e-2_f64 * t4865 - 0.15538616723388920628e-3_f64 * t822 + 0.18204739583333333333e-4_f64 * t4881;
    (t8965, t8969, t8972, t8979, t8996)
}
