//! GGA_C_ACGGAP lxc pol — lxc_pol part 14 (v4rho3sigma_6) CSE chunk 811/1223 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part14_v4rho3sigma_6_chunk811<F: Float>(t8556: F, t8574: F, t8580: F, t8582: F, t8607: F, t8619: F, t8625: F, t8650: F, t8680: F, t8682: F, t8684: F, t8690: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
    let t9206 = F::cast_from(0.10482697429868050689e-2_f64) * t8556;
    let t9211 = F::cast_from(0.85748036236139473944e-3_f64) * t8574;
    let t9214 = F::cast_from(0.18868855373762491241e-2_f64) * t8580;
    let t9215 = F::cast_from(0.21437009059034868486e-3_f64) * t8582;
    let t9222 = F::cast_from(0.42874018118069736972e-3_f64) * t8607;
    let t9226 = F::new(0.28015625e-1) * t8619;
    let t9228 = F::new(7.0) / F::new(144.0) * t8625;
    let t9239 = F::cast_from(0.10718504529517434243e-2_f64) * t8650;
    let t9248 = F::new(11.0) / F::new(192.0) * t8680;
    let t9249 = F::new(11.0) / F::new(576.0) * t8682;
    let t9250 = F::new(7.0) / F::new(72.0) * t8684;
    let t9252 = F::cast_from(0.21437009059034868486e-3_f64) * t8690;
    (t9206, t9211, t9214, t9215, t9222, t9226, t9228, t9239, t9248, t9249, t9250, t9252)
}
