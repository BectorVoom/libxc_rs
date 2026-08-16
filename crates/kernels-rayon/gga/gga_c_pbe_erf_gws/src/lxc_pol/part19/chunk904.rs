//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 19 (v4rho3sigma_7) CSE chunk 904/1404 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part19_v4rho3sigma_7_chunk904(t1563: f64, t3637: f64, t102: f64, t3656: f64, t481: f64, t2873: f64, t974: f64, t3660: f64, t10102: f64, t10106: f64, t10107: f64, t10110: f64, t127: f64, t2893: f64, t5836: f64, t8200: f64) -> (f64, f64, f64, f64) {
    let t10117 = t1563 * t3637;
    let t10123 = 0.1753815e2_f64 * t102 * t3656 * t481;
    let t10126 = 0.116921e2_f64 * t102 * t974 * t2873;
    let t10129 = 0.584605e1_f64 * t102 * t3660 * t481;
    let t10130 = -t10102 - 4.0_f64 / 9.0_f64 * t8200 + t5836 - t10106 - 0.146904e1_f64 * t127 * t10107 - 0.293808e2_f64 * t127 * t10110 * t481 + 0.1175232e2_f64 * t127 * t2893 * t2873 + 0.587616e1_f64 * t127 * t10117 * t481 - t10123 + t10126 + t10129;
    (t10123, t10126, t10129, t10130)
}
