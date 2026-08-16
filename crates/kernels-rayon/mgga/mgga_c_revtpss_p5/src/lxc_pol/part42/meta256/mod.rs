//! MGGA_C_REVTPSS lxc pol kernel — _part42_v4rho3tau_5 meta256 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;
mod chunk5;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_chunk978;
use chunk1::mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_chunk979;
use chunk2::mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_chunk980;
use chunk3::mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_chunk981;
use chunk4::mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_chunk982;
use chunk5::mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_chunk983;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_meta256(t1911: f64, t2198: f64, t1312: f64, t2199: f64, t2201: f64, t4248: f64, t651: f64, t7732: f64, t7889: f64, t8393: f64, t8407: f64, t8411: f64, t3: f64, param_d: f64, t1518: f64, t8342: f64, t117: f64, t8406: f64, t1916: f64, t1918: f64, t2207: f64, t2209: f64, t572: f64, t573: f64, t587: f64, t65: f64, t143: f64, t2580: f64, t130: f64, t2566: f64, t700: f64, t2584: f64, t121: f64, t131: f64, t141: f64, t22: f64, t2456: f64, t624: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let t8413 = mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_chunk978(t1911, t2198);
        let (t8416, t8417) = mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_chunk979(t1312, t2199, t2201, t4248, t651, t7732, t7889, t8393, t8407, t8411, t8413, t3);
        let t8421 = mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_chunk980(t8416, param_d);
        let (t8427, t8430, t8433, t8779) = mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_chunk981(t1518, t8342, t117, t8406, t1916, t1918, t2207, t2209, t572, t573, t8421, t587, t65);
        let (t9275, t9278) = mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_chunk982(t143, t2580, t130, t2566, t700, t2584);
        let (t9283, t9285) = mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_chunk983(t121, t131, t141, t22, t2456, t624);
    (t8413, t8416, t8417, t8421, t8427, t8430, t8433, t8779, t9275, t9278, t9283, t9285)
}
