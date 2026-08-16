//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 18 (v4rho3sigma_6) CSE chunk 1076/1389 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part18_v4rho3sigma_6_chunk1076(t12092: f64, t12056: f64, t12057: f64, t12060: f64, t12061: f64, t12067: f64, t12071: f64, t12078: f64, t12082: f64, t12086: f64, t12088: f64, t2277: f64, t6718: f64, t9669: f64) -> (f64, f64) {
    let t12093 = 7.0_f64 / 72.0_f64 * t12092;
    let t12094 = -t12056 + 7.0_f64 / 2304.0_f64 * t12057 + 119.0_f64 / 3456.0_f64 * t9669 + t12060 - 7.0_f64 / 2304.0_f64 * t12061 - t12067 - t12071 + t12078 + t12082 - t12086 - t2277 * t12088 / 1536.0_f64 + 119.0_f64 / 6912.0_f64 * t6718 - t12093;
    (t12093, t12094)
}
