//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 992/1302 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part11_v4rho4_4_chunk992(t21780: f64, t3739: f64, t19804: f64, t3912: f64, t19637: f64, t2242: f64, t3893: f64, t3724: f64, t3903: f64, t4442: f64, t3889: f64, t3916: f64, t6792: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t35941 = t21780 * t3739;
    let t36041 = t3912 * t19804;
    let t36114 = t3912 * t19637;
    let t36152 = t2242 * t3893;
    let t36244 = t2242 * t3724;
    let t36246 = t4442 * t3903;
    let t36290 = t2242 * t3889;
    let t36323 = t3916 * t6792;
    (t35941, t36041, t36114, t36152, t36244, t36246, t36290, t36323)
}
