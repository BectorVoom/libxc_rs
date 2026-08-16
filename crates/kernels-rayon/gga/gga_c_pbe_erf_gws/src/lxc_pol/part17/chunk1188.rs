//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 17 (v4rho3sigma_5) CSE chunk 1188/1352 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part17_v4rho3sigma_5_chunk1188(t13888: f64, t2352: f64, t353: f64, t859: f64, t1178: f64, t13918: f64, t13909: f64, t892: f64, t2416: f64, t4052: f64, t938: f64, t13808: f64, t13906: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t51063 = t859 * t353 * t13888 * t2352;
    let t51066 = t1178 * t13918;
    let t51081 = t859 * t892 * t13909;
    let t51084 = t2416 * t4052;
    let t51087 = t859 * t353 * t51084 * t938;
    let t51096 = t13808 * t13906;
    (t51063, t51066, t51081, t51084, t51087, t51096)
}
