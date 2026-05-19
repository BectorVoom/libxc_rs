//! GGA_C_ACGGAP lxc pol — lxc_pol part 13 (v4rho3sigma_5) CSE chunk 1104/1213 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part13_v4rho3sigma_5_chunk1104<F: Float>(t142: F, t5187: F, t8888: F, t507: F, t7436: F, t961: F, t30978: F, t30982: F, t30985: F, t30987: F, t30949: F, t30956: F, t30963: F, t30967: F, t30974: F, t30976: F, t30980: F, t35139: F, t35146: F, t35149: F, t35151: F) -> F {
    let t35154 = t8888 * t142 * t5187;
    let t35157 = t7436 * t507 * t961;
    let t35160 = F::cast_from(0.16006300097412701803e-1_f64) * t30978;
    let t35162 = F::cast_from(0.16006300097412701803e-1_f64) * t30982;
    let t35163 = F::cast_from(0.21437009059034868486e-2_f64) * t30985;
    let t35164 = F::cast_from(0.25724410870841842184e-2_f64) * t30987;
    let t35165 = -F::cast_from(0.16006300097412701803e-1_f64) * t30949 - F::cast_from(0.21437009059034868486e-3_f64) * t35139 + F::cast_from(0.21437009059034868486e-3_f64) * t30956 + F::cast_from(0.42874018118069736972e-3_f64) * t30963 - F::cast_from(0.7145669686344956162e-4_f64) * t30967 + F::cast_from(0.15724046144802076034e-3_f64) * t30974 - t35146 - t35149 + t35151 / F::new(24.0) + t35154 / F::new(24.0) + t35157 / F::new(24.0) - F::cast_from(0.32012600194825403606e-1_f64) * t30976 + t35160 + F::cast_from(0.16006300097412701803e-1_f64) * t30980 - t35162 + t35163 - t35164;
    t35165
}
