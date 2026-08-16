//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 19 (v4rho3sigma_7) CSE chunk 1175/1404 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part19_v4rho3sigma_7_chunk1175(t1109: f64, t2118: f64, t1113: f64, t3975: f64, t3972: f64, t1076: f64, t331: f64, t1123: f64, t850: f64, t833: f64, t12109: f64, t2409: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t15149 = t2118 * t1109;
    let t15150 = t1113 * t15149;
    let t15151 = t3975 * t15150;
    let t15152 = t3972 * t15151;
    let t15159 = t1076 * t331;
    let t15161 = t850 * t1123 * t15159;
    let t15162 = t15161 * t833;
    let t15164 = t2409 * t12109;
    (t15149, t15151, t15152, t15159, t15161, t15162, t15164)
}
