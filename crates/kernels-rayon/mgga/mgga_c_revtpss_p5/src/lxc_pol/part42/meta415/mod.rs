//! MGGA_C_REVTPSS lxc pol kernel — _part42_v4rho3tau_5 meta415 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_chunk1471;
use chunk1::mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_chunk1472;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_meta415(t2195: f64, t2289: f64, t31027: f64, t8312: f64, t31032: f64, t8316: f64, t104: f64, t2357: f64, t116: f64, t8320: f64, t10199: f64, t655: f64, t2198: f64, t5787: f64, t5517: f64, t1312: f64, t13426: f64, t18227: f64, t2199: f64, t2201: f64, t2322: f64, t27123: f64, t27126: f64, t28219: f64, t4248: f64, t4254: f64, t5523: f64, t651: f64, t7732: f64, t7889: f64, t8307: f64, t8321: f64, t8325: f64, t8327: f64, t8393: f64, t8411: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t31134, t31135, t31137, t31149, t31234, t31287) = mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_chunk1471(t2195, t2289, t31027, t8312, t31032, t8316, t104, t2357, t116, t8320, t10199, t655);
        let (t31382, t31390, t31398) = mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_chunk1472(t2198, t5787, t5517, t1312, t13426, t18227, t2199, t2201, t2322, t27123, t27126, t28219, t4248, t4254, t5523, t651, t7732, t7889, t8307, t8321, t8325, t8327, t8393, t8411);
    (t31134, t31135, t31137, t31149, t31234, t31287, t31382, t31390, t31398)
}
