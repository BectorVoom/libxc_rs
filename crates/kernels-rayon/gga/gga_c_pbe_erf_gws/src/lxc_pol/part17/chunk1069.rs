//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 17 (v4rho3sigma_5) CSE chunk 1069/1352 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part17_v4rho3sigma_5_chunk1069(t1143: f64, t2416: f64, t1105: f64, t2053: f64, t944: f64, t4058: f64, t945: f64, t1172: f64, t318: f64, t2182: f64, t3944: f64, t810: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t12213 = t1143 * t2416;
    let t12275 = t2053 * t1105;
    let t12276 = t12275 * t944;
    let t13751 = t4058 * t945;
    let t13756 = t1172 * t318;
    let t13757 = t3944 * t2182;
    let t13760 = t13751 * t810;
    (t12213, t12275, t12276, t13751, t13756, t13757, t13760)
}
