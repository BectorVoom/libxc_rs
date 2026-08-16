//! MGGA_C_REVTPSS lxc pol kernel — _part28_v4rho3sigma_3 meta147 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk795;
use chunk1::mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk796;
use chunk2::mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk797;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_meta147(t1025: f64, t3215: f64, t3075: f64, t373: f64, t371: f64, t372: f64, t225: f64, t3046: f64, t366: f64, t362: f64, t40: f64, t611: f64, t361: f64, t351: f64, t1054: f64, t1058: f64, t1014: f64, t2857: f64, t2251: f64, t1012: f64, t1010: f64, t614: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t3216, t3218, t3220, t3223) = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk795(t1025, t3215, t3075, t373, t371, t372, t225, t3046);
        let t3224 = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk796(t3223, t366);
        let (t3229, t3230, t3231, t3234, t3237, t3238, t3241) = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk797(t362, t40, t611, t361, t351, t1054, t1058, t1014, t2857, t2251, t1012, t1010, t614);
    (t3216, t3218, t3220, t3223, t3224, t3229, t3230, t3231, t3234, t3237, t3238, t3241)
}
