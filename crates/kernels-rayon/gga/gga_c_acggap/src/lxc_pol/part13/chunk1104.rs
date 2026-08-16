//! GGA_C_ACGGAP lxc pol — lxc_pol part 13 (v4rho3sigma_5) CSE chunk 1104/1213 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part13_v4rho3sigma_5_chunk1104(t142: f64, t5187: f64, t8888: f64, t507: f64, t7436: f64, t961: f64, t30978: f64, t30982: f64, t30985: f64, t30987: f64, t30949: f64, t30956: f64, t30963: f64, t30967: f64, t30974: f64, t30976: f64, t30980: f64, t35139: f64, t35146: f64, t35149: f64, t35151: f64) -> f64 {
    let t35154 = t8888 * t142 * t5187;
    let t35157 = t7436 * t507 * t961;
    let t35160 = 0.16006300097412701803e-1_f64 * t30978;
    let t35162 = 0.16006300097412701803e-1_f64 * t30982;
    let t35163 = 0.21437009059034868486e-2_f64 * t30985;
    let t35164 = 0.25724410870841842184e-2_f64 * t30987;
    let t35165 = -0.16006300097412701803e-1_f64 * t30949 - 0.21437009059034868486e-3_f64 * t35139 + 0.21437009059034868486e-3_f64 * t30956 + 0.42874018118069736972e-3_f64 * t30963 - 0.7145669686344956162e-4_f64 * t30967 + 0.15724046144802076034e-3_f64 * t30974 - t35146 - t35149 + t35151 / 24.0_f64 + t35154 / 24.0_f64 + t35157 / 24.0_f64 - 0.32012600194825403606e-1_f64 * t30976 + t35160 + 0.16006300097412701803e-1_f64 * t30980 - t35162 + t35163 - t35164;
    t35165
}
