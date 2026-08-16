//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 18 (v4rho3sigma_6) CSE chunk 557/1389 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part18_v4rho3sigma_6_chunk557(t133: f64, t1511: f64, t1519: f64, t1583: f64, t1584: f64, t2862: f64, t2865: f64, t2868: f64, t2876: f64, t2886: f64, t2909: f64, t2911: f64, t2912: f64, t2913: f64) -> f64 {
    let t2919 = -t1511 + t2862 + t1519 + t2865 + t2868 - t2876 + t1583 + 0.57475166666666666666e0_f64 * t1584 + 0.57475166666666666667e0_f64 * t2909 + 0.5172765e1_f64 * t2911 * t2912 * t2913 - 0.1724255e1_f64 * t133 * t2886;
    t2919
}
