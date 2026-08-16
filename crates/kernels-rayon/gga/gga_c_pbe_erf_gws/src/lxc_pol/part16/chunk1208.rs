//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 16 (v4rho3sigma_4) CSE chunk 1208/1361 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part16_v4rho3sigma_4_chunk1208(t14175: f64, t804: f64, t14360: f64, t2053: f64, t1211: f64, t21885: f64, t8556: f64, t14365: f64, t321: f64, t14372: f64, t14185: f64, t2352: f64, t353: f64, t859: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t52092 = t804 * t14175;
    let t52094 = t14360 * t2053;
    let t52105 = t1211 * t21885;
    let t52112 = t804 * t1211;
    let t52113 = t52112 * t8556;
    let t52115 = t321 * t14365;
    let t52127 = t321 * t14372;
    let t52131 = t859 * t353 * t14185 * t2352;
    (t52092, t52094, t52105, t52112, t52113, t52115, t52127, t52131)
}
