//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 16 (v4rho3sigma_4) CSE chunk 1070/1361 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part16_v4rho3sigma_4_chunk1070(t1143: f64, t2416: f64, t1105: f64, t2053: f64, t944: f64, t1172: f64, t318: f64, t810: f64, t254: f64, t932: f64, t3970: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t12213 = t1143 * t2416;
    let t12275 = t2053 * t1105;
    let t12276 = t12275 * t944;
    let t13756 = t1172 * t318;
    let t13763 = t810 * t944;
    let t13775 = t932 * t254;
    let t13776 = t3970 * t13775;
    (t12213, t12275, t12276, t13756, t13763, t13775, t13776)
}
