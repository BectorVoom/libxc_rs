//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 18 (v4rho3sigma_6) CSE chunk 908/1389 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part18_v4rho3sigma_6_chunk908(t10051: f64, t10054: f64, t10090: f64, t10154: f64, t10158: f64, t10162: f64, t2911: f64, t2912: f64, t5753: f64, t5755: f64, t5776: f64, t5863: f64, t5864: f64, t8137: f64, t8142: f64, t8231: f64) -> f64 {
    let t10167 = t5753 - t5755 - t10051 + t10054 - 0.2069106e2_f64 * t2911 * t8231 * t10154 + 0.1034553e2_f64 * t2911 * t2912 * t10158 + 0.5172765e1_f64 * t2911 * t2912 * t10162 - t5863 - t5776 - t8137 + t8142 + t10090 - 0.76633555555555555554e0_f64 * t5864;
    t10167
}
