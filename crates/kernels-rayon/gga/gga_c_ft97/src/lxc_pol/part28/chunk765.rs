//! GGA_C_FT97 lxc pol — lxc_pol part 28 (v4rho2sigma2_6) CSE chunk 765/1189 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part28_v4rho2sigma2_6_chunk765(t32355: f64, t5508: f64, t28: f64, t1308: f64, t5748: f64, t376: f64, t7167: f64, t1286: f64, t32338: f64, t22917: f64, t5507: f64, t1332: f64, t5743: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t32391 = t32355 * t5508;
    let t32392 = t28 * t32391;
    let t32395 = t1308 * t5748;
    let t32396 = t28 * t32395;
    let t32399 = t376 * t7167;
    let t32401 = t1286 * t32399 / 9.0_f64;
    let t32402 = t32338 * t5508;
    let t32403 = t28 * t32402;
    let t32405 = t5507 * t22917;
    let t32406 = t28 * t32405;
    let t32411 = t1332 * t5743;
    (t32391, t32392, t32395, t32396, t32399, t32401, t32402, t32403, t32405, t32406, t32411)
}
