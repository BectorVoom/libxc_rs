//! GGA_C_FT97 lxc pol — lxc_pol part 15 (v4rho4_4) CSE chunk 663/1222 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part15_v4rho4_4_chunk663(t19100: f64, t4092: f64, t39: f64, t817: f64, t1200: f64, t800: f64, t285: f64, t5249: f64, t8959: f64, t4939: f64, t703: f64, t1196: f64, t284: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t19101 = t4092 * t19100;
    let t19106 = t817 * t39;
    let t19107 = t1200 * t19106;
    let t19132 = t800 * t19100;
    let t19135 = t285 * t19106;
    let t19167 = 0.8854768453090786061e-3_f64 * t8959 * t5249;
    let t19168 = t703 * t4939;
    let t19233 = t1196 * t284;
    (t19101, t19106, t19107, t19132, t19135, t19167, t19168, t19233)
}
