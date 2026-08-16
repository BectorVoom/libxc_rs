//! GGA_C_GAPLOC lxc pol — lxc_pol part 46 (v4rhosigma3_11) CSE chunk 704/1029 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part46_v4rhosigma3_11_chunk704(t12903: f64, t12933: f64, t12956: f64, t12999: f64, t502: f64, t3263: f64, t8862: f64, t2969: f64, t3322: f64, t10800: f64, t977: f64, t11004: f64, t935: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t13001 = t12903 + t12933 + t12956 + t12999;
    let t13002 = t502 * t13001;
    let t13004 = 2.0_f64 * t8862 * t3263;
    let t13005 = t2969 * t3322;
    let t13006 = t10800 * t977;
    let t13008 = t11004 * t935;
    (t13001, t13002, t13004, t13005, t13006, t13008)
}
