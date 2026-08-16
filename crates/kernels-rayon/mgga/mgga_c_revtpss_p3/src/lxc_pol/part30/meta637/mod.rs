//! MGGA_C_REVTPSS lxc pol kernel — _part30_v4rho3sigma_5 meta637 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk2206;
use chunk1::mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk2207;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_meta637(t1459: f64, t28277: f64, t28280: f64, t5795: f64, t7331: f64, t28268: f64, t116: f64, t28042: f64, t572: f64, t670: f64, t2371: f64, t28276: f64, t4158: f64, t7950: f64, t18190: f64, t2042: f64, t28271: f64, t7334: f64, t1518: f64, t95137: f64, t26123: f64, t4292: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t101613, t101617, t101619, t101621, t101625, t101628) = mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk2206(t1459, t28277, t28280, t5795, t7331, t28268, t116, t28042, t572, t670, t2371, t28276);
        let (t101632, t101634, t101640, t101642, t101645, t101648) = mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk2207(t4158, t7950, t18190, t2042, t1459, t28271, t5795, t7334, t1518, t572, t95137, t26123, t4292);
    (t101613, t101617, t101619, t101621, t101625, t101628, t101632, t101634, t101640, t101642, t101645, t101648)
}
