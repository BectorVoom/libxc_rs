//! GGA_C_ACGGAP lxc pol — lxc_pol part 11 (v4rho3sigma_3) CSE chunk 1131/1213 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part11_v4rho3sigma_3_chunk1131<F: Float>(t31285: F, t4360: F, t7741: F, t13287: F, t34823: F, t34828: F, t31312: F, t31316: F, t31322: F, t31284: F, t31287: F, t31291: F, t31293: F, t31296: F, t31297: F, t31299: F, t31305: F, t31318: F, t31341: F, t31342: F, t31344: F) -> F {
    let t35527 = F::cast_from(0.10718504529517434243e-2_f64) * t31285;
    let t35529 = t7741 * t4360;
    let t35535 = t34823 * t13287 * t34828;
    let t35538 = F::cast_from(0.85748036236139473944e-3_f64) * t31312;
    let t35539 = F::cast_from(0.12579236915841660827e-2_f64) * t31316;
    let t35541 = F::cast_from(0.85748036236139473944e-3_f64) * t31322;
    let t35544 = -t31284 - t35527 - F::cast_from(0.68598428988911579156e-2_f64) * t31287 + t31291 - F::cast_from(0.34299214494455789578e-2_f64) * t35529 + t31293 / F::new(32.0) - t31296 - F::cast_from(0.31448092289604152068e-2_f64) * t31297 + F::cast_from(0.28303283060643736861e-1_f64) * t31299 - F::cast_from(0.85748036236139473944e-3_f64) * t35535 - F::cast_from(0.40015750243531754508e-2_f64) * t31305 - t35538 + t35539 + F::cast_from(0.11321313224257494744e-1_f64) * t31318 + t35541 + t31341 + F::new(7.0) / F::new(144.0) * t31342 + F::new(7.0) / F::new(288.0) * t31344;
    t35544
}
