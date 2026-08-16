//! MGGA_C_REVTPSS lxc pol kernel — _part30_v4rho3sigma_5 meta633 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk2201;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_meta633(t25188: f64, t7937: f64, t1936: f64, t49686: f64, t75667: f64, t13426: f64, t7002: f64, t75485: f64, t18227: f64, t25832: f64, t4248: f64, t98484: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t101486, t101504, t101506, t101508, t101510, t101512, t101514, t101517) = mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk2201(t25188, t7937, t1936, t49686, t75667, t13426, t7002, t75485, t18227, t25832, t4248, t98484);
    (t101486, t101504, t101506, t101508, t101510, t101512, t101514, t101517)
}
