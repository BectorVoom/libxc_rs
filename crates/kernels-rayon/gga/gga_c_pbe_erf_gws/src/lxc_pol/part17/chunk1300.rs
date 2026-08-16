//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 17 (v4rho3sigma_5) CSE chunk 1300/1352 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part17_v4rho3sigma_5_chunk1300(t1125: f64, t51317: f64, t4039: f64, t9411: f64, t28139: f64, t850: f64, t14093: f64, t51306: f64, t9609: f64, t3065: f64, t3167: f64, t2134: f64) -> (f64, f64, f64, f64, f64) {
    let t54075 = t1125 * t51317;
    let t54077 = t4039 * t9411;
    let t54079 = t850 * t28139;
    let t54080 = t54079 * t14093;
    let t54082 = t51306 * t9609;
    let t54084 = t3065 * t3167;
    let t54085 = t2134 * t54084;
    (t54075, t54077, t54080, t54082, t54085)
}
