//! GGA_C_FT97 lxc pol — lxc_pol part 2 (v3rho3_1) CSE chunk 290/1007 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part2_v3rho3_1_chunk290(t1095: f64, t278: f64, t1190: f64, t274: f64, t807: f64, t291: f64, t800: f64, t281: f64, t283: f64, t1121: f64, t1125: f64, t818: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t1193 = t1095 * t278;
    let t1196 = 0.23410285231011484e0_f64 * t1190 * t274 - 0.532971647967385935e-1_f64 * t807 * t1193;
    let t1197 = t291 * t1196;
    let t1198 = t800 * t1197;
    let t1200 = t281 * t283;
    let t1201 = t1200 * t291;
    let t1208 = -0.13335600218518518519e0_f64 * t1121 + t818 + 0.16669500273148148149e-1_f64 * t1125;
    (t1193, t1196, t1197, t1198, t1200, t1201, t1208)
}
