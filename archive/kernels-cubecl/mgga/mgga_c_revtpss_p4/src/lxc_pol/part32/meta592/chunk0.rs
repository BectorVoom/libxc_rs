//! MGGA_C_REVTPSS lxc pol — lxc_pol part 32 (v4rho3sigma_7) CSE chunk 1924/2056 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1924<F: Float>(t98968: F, t98972: F, t98983: F, t98991: F, t99000: F, t99006: F, t99011: F, t99019: F, t99021: F, t99023: F, t99026: F, t99029: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
    let t103265 = F::cast_from(0.11433071498151929859e-2_f64) * t98968;
    let t103267 = F::cast_from(0.4065600224742826258e-3_f64) * t98972;
    let t103273 = F::cast_from(0.4065600224742826258e-3_f64) * t98983;
    let t103276 = F::cast_from(0.80031500487063509014e-2_f64) * t98991;
    let t103280 = F::cast_from(0.22866142996303859718e-3_f64) * t99000;
    let t103283 = F::cast_from(0.57165357490759649296e-4_f64) * t99006;
    let t103286 = F::cast_from(0.32012600194825403606e-1_f64) * t99011;
    let t103290 = F::cast_from(0.4065600224742826258e-3_f64) * t99019;
    let t103291 = F::cast_from(0.10164000561857065645e-3_f64) * t99021;
    let t103292 = F::cast_from(0.32012600194825403606e-1_f64) * t99023;
    let t103293 = F::cast_from(0.22866142996303859718e-3_f64) * t99026;
    let t103294 = F::cast_from(0.57165357490759649296e-4_f64) * t99029;
    (t103265, t103267, t103273, t103276, t103280, t103283, t103286, t103290, t103291, t103292, t103293, t103294)
}
