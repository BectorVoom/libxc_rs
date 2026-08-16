//! MGGA_C_REVTPSS lxc pol kernel — _part40_v4rho3tau_3 meta260 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part40_v4rho3tau_3_chunk974;
use chunk1::mgga_c_revtpss_lxc_pol_part40_v4rho3tau_3_chunk975;
use chunk2::mgga_c_revtpss_lxc_pol_part40_v4rho3tau_3_chunk976;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part40_v4rho3tau_3_meta260(t508: f64, t8406: f64, t569: f64, t1911: f64, t2198: f64, t1312: f64, t2199: f64, t2201: f64, t4248: f64, t651: f64, t7732: f64, t7889: f64, t8393: f64, t3: f64, t1518: f64, t8342: f64, t117: f64, t1916: f64, t1918: f64, t2207: f64, t2209: f64, t572: f64, t573: f64, param_d: f64, t587: f64, t65: f64, t143: f64, t2580: f64, t130: f64, t2566: f64, t700: f64, t2584: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t8407, t8411, t8413, t8416) = mgga_c_revtpss_lxc_pol_part40_v4rho3tau_3_chunk974(t508, t8406, t569, t1911, t2198, t1312, t2199, t2201, t4248, t651, t7732, t7889, t8393);
        let (t8417, t8421, t8427, t8430, t8433) = mgga_c_revtpss_lxc_pol_part40_v4rho3tau_3_chunk975(t3, t8416, t1518, t8342, t117, t8406, t1916, t1918, t2207, t2209, t572, t573, param_d);
        let (t8779, t9275, t9278) = mgga_c_revtpss_lxc_pol_part40_v4rho3tau_3_chunk976(t587, t65, t143, t2580, t130, t2566, t700, t2584);
    (t8407, t8411, t8413, t8416, t8417, t8421, t8427, t8430, t8433, t8779, t9275, t9278)
}
