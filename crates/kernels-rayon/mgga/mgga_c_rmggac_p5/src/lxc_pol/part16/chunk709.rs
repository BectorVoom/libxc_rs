//! MGGA_C_RMGGAC lxc pol — lxc_pol part 16 (v4rho3sigma_7) CSE chunk 709/1158 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part16_v4rho3sigma_7_chunk709(t9818: f64, t9820: f64, t530: f64, t9343: f64, t2211: f64, t6557: f64, t884: f64, t1763: f64, t8041: f64, t1356: f64, t9827: f64, t9832: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t10302 = 0.2553875993597870364e-4_f64 * t9818;
    let t10303 = 0.1702583995731913576e-4_f64 * t9820;
    let t10304 = t530 * t9343;
    let t10305 = 0.4726e1_f64 * t10304;
    let t10306 = t2211 * t6557;
    let t10307 = t884 * t10306;
    let t10308 = 0.23948483403727617128e0_f64 * t10307;
    let t10309 = t8041 * t1763;
    let t10310 = t1356 * t10309;
    let t10311 = 0.11974241701863808564e0_f64 * t10310;
    let t10312 = 0.85129199786595678799e-5_f64 * t9827;
    let t10313 = 0.13637330827122670865e0_f64 * t9832;
    (t10302, t10303, t10305, t10306, t10308, t10309, t10311, t10312, t10313)
}
