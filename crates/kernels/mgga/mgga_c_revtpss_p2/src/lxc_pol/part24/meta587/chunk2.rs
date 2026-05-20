//! MGGA_C_REVTPSS lxc pol — lxc_pol part 24 (v4rho4_4) CSE chunk 1826/1850 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1826<F: Float>(t46800: F, t46810: F, t46817: F, t46820: F, t46824: F, t46831: F, t46840: F, t48792: F, t74429: F, t74437: F, t85873: F, t85885: F, t86061: F, t86070: F, t86074: F, t86078: F, t86080: F) -> F {
    let t92136 = t46800 + t46810 - t46817 + t46820 - t46824 + F::new(7.0) / F::new(36.0) * t85873 - F::cast_from(0.17149607247227894789e-3_f64) * t85885 - F::cast_from(0.50820002809285328224e-4_f64) * t86061 + F::cast_from(0.17149607247227894789e-2_f64) * t86070 - F::cast_from(0.30492001685571196935e-3_f64) * t86074 + F::cast_from(0.30492001685571196935e-3_f64) * t86078 + F::cast_from(0.40015750243531754508e-2_f64) * t86080 - t46831 + t46840 + F::cast_from(0.81312004494856525159e-3_f64) * t74429 - F::cast_from(0.51384669507166276316e-2_f64) * t48792 - F::cast_from(0.1084295579938911763e-3_f64) * t74437;
    t92136
}
