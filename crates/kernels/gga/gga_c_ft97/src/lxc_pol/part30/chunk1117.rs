//! GGA_C_FT97 lxc pol — lxc_pol part 30 (v4rho2sigma2_11) CSE chunk 1117/1184 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part30_v4rho2sigma2_11_chunk1117<F: Float>(t28548: F, t33404: F, t28667: F, t33414: F, t111830: F, t111838: F, t112071: F, t112156: F, t112159: F, t127360: F, t142712: F, t150378: F, t150876: F, t150879: F, t152981: F, t152984: F, t152987: F, t152996: F, t153007: F, t28558: F, t28603: F) -> (F, F, F) {
    let t153011 = t33404 * t28548;
    let t153017 = t33414 * t28667;
    let t153020 = F::cast_from(0.54377463984153450669e0_f64) * t127360 * t152981 + F::cast_from(0.6041940442683716741e-1_f64) * t112156 * t152984 - F::cast_from(0.6041940442683716741e-1_f64) * t112159 * t152987 - F::cast_from(0.6041940442683716741e-1_f64) * t111830 * t152984 - F::cast_from(0.6041940442683716741e-1_f64) * t28558 * t150876 + F::cast_from(0.40279602951224778273e-1_f64) * t28558 * t150879 - F::cast_from(0.6041940442683716741e-1_f64) * t111830 * t152996 + F::cast_from(0.6041940442683716741e-1_f64) * t112156 * t152996 + F::cast_from(0.6041940442683716741e-1_f64) * t28603 * t150876 + F::cast_from(0.6041940442683716741e-1_f64) * t28603 * t150378 - F::cast_from(0.6041940442683716741e-1_f64) * t28558 * t150378 + F::cast_from(0.10069900737806194568e-1_f64) * t153007 - F::cast_from(0.40279602951224778273e-1_f64) * t28603 * t150879 - F::cast_from(0.6041940442683716741e-1_f64) * t112159 * t153011 + F::cast_from(0.6041940442683716741e-1_f64) * t111838 * t152987 - F::cast_from(0.10069900737806194568e-1_f64) * t142712 + F::cast_from(0.36251642656102300446e0_f64) * t112071 * t153017;
    (t153011, t153017, t153020)
}
