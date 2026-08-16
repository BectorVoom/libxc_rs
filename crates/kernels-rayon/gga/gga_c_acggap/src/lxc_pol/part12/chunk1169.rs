//! GGA_C_ACGGAP lxc pol — lxc_pol part 12 (v4rho3sigma_4) CSE chunk 1169/1250 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part12_v4rho3sigma_4_chunk1169(t34361: f64, t30365: f64, t30369: f64, t30375: f64, t30387: f64, t30398: f64, t30412: f64, t30416: f64, t30444: f64, t30448: f64, t30452: f64, t30457: f64, t30459: f64, t32456: f64, t32458: f64, t32461: f64, t32462: f64, t34371: f64) -> f64 {
    let t37047 = 0.25724410870841842184e-1_f64 * t34361;
    let t37058 = -0.17149607247227894789e-2_f64 * t30365 + 0.41930789719472202759e-2_f64 * t30369 + 0.25158473831683321656e-2_f64 * t30375 - t37047 + 11.0_f64 / 192.0_f64 * t30387 - t32456 + 35.0_f64 / 108.0_f64 * t30398 - t32458 + 0.12579236915841660828e-1_f64 * t30412 - 0.50316947663366643309e-2_f64 * t30416 + t32461 + 0.36675e0_f64 * t34371 + t32462 - 0.31448092289604152068e-2_f64 * t30444 - 0.12862205435420921092e-2_f64 * t30448 + 0.12579236915841660828e-2_f64 * t30452 - 0.18007087609589289529e-1_f64 * t30457 + 0.85748036236139473944e-3_f64 * t30459;
    t37058
}
