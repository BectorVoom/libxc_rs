//! MGGA_C_REVTPSS lxc pol — lxc_pol part 29 (v4rho3sigma_4) CSE chunk 2013/2049 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk2013<F: Float>(t99009: F, t99011: F, t99013: F, t99019: F, t99021: F, t99023: F, t99026: F, t99029: F, t99033: F, t99035: F, t99015: F, t99017: F, t99031: F) -> F {
    let t103285 = F::cast_from(0.90702367218671976884e-1_f64) * t99009;
    let t103286 = F::cast_from(0.32012600194825403606e-1_f64) * t99011;
    let t103287 = F::cast_from(0.2168320119862840671e-2_f64) * t99013;
    let t103290 = F::cast_from(0.4065600224742826258e-3_f64) * t99019;
    let t103291 = F::cast_from(0.10164000561857065645e-3_f64) * t99021;
    let t103292 = F::cast_from(0.32012600194825403606e-1_f64) * t99023;
    let t103293 = F::cast_from(0.22866142996303859718e-3_f64) * t99026;
    let t103294 = F::cast_from(0.57165357490759649296e-4_f64) * t99029;
    let t103296 = F::cast_from(0.80031500487063509014e-2_f64) * t99033;
    let t103297 = F::cast_from(0.22675591804667994221e-1_f64) * t99035;
    let t103298 = -t103285 + t103286 + t103287 + F::cast_from(0.34299214494455789578e-2_f64) * t99015 - F::cast_from(0.17149607247227894789e-2_f64) * t99017 + t103290 - t103291 - t103292 - t103293 + t103294 - F::cast_from(0.10289764348336736873e0_f64) * t99031 + t103296 - t103297;
    t103298
}
