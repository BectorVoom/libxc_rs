//! GGA_C_ACGGAP lxc pol — lxc_pol part 12 (v4rho3sigma_4) CSE chunk 1192/1250 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part12_v4rho3sigma_4_chunk1192(t35286: f64, t35290: f64, t35301: f64, t35315: f64, t35317: f64, t31105: f64, t35294: f64, t35298: f64, t35305: f64, t35307: f64, t35309: f64, t35311: f64, t35319: f64, t35321: f64, t35327: f64, t35331: f64, t35335: f64) -> f64 {
    let t37475 = 0.85748036236139473944e-3_f64 * t35286;
    let t37476 = 0.42874018118069736972e-3_f64 * t35290;
    let t37479 = 0.31448092289604152068e-2_f64 * t35301;
    let t37484 = 0.12862205435420921092e-1_f64 * t35315;
    let t37485 = 0.34299214494455789578e-2_f64 * t35317;
    let t37491 = 0.75475421495049964963e-2_f64 * t31105 - t37475 + t37476 + 0.12579236915841660828e-2_f64 * t35294 - 0.25724410870841842184e-2_f64 * t35298 + t37479 + 0.10718504529517434243e-2_f64 * t35305 - 0.51448821741683684366e-2_f64 * t35307 + 0.13719685797782315831e-1_f64 * t35309 + 0.68598428988911579156e-2_f64 * t35311 - t37484 - t37485 + 0.34299214494455789578e-2_f64 * t35319 - 0.13719685797782315831e-1_f64 * t35321 + 0.18868855373762491241e-2_f64 * t35327 - 0.18868855373762491242e-2_f64 * t35331 - 0.12862205435420921092e-2_f64 * t35335;
    t37491
}
