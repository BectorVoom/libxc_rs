//! MGGA_C_REVTPSS lxc pol kernel — _part40_v4rho3tau_3 meta404 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part40_v4rho3tau_3_chunk1480;
use chunk1::mgga_c_revtpss_lxc_pol_part40_v4rho3tau_3_chunk1481;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part40_v4rho3tau_3_meta404(t3: f64, t31204: f64, t2198: f64, t2327: f64, t116: f64, t8320: f64, t670: f64, t2371: f64, t8342: f64, t117: f64, t31157: f64, t1459: f64, t1461: f64, t2207: f64, t2209: f64, t4158: f64, t4162: f64, t4165: f64, t572: f64, t573: f64, t8336: f64, t8343: f64, t8346: f64, param_d: f64, t10199: f64, t655: f64, t5787: f64, t5517: f64, t1312: f64, t13426: f64, t18227: f64, t2199: f64, t2201: f64, t2322: f64, t27123: f64, t27126: f64, t28219: f64, t4248: f64, t4254: f64, t5523: f64, t651: f64, t7732: f64, t7889: f64, t8307: f64, t8321: f64, t8325: f64, t8327: f64, t8393: f64, t8411: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t31205, t31217, t31231, t31234, t31235, t31238, t31241, t31244) = mgga_c_revtpss_lxc_pol_part40_v4rho3tau_3_chunk1480(t3, t31204, t2198, t2327, t116, t8320, t670, t2371, t8342, t117, t31157, t1459, t1461, t2207, t2209, t4158, t4162, t4165, t572, t573, t8336, t8343, t8346, param_d);
        let (t31287, t31382, t31390, t31398) = mgga_c_revtpss_lxc_pol_part40_v4rho3tau_3_chunk1481(t10199, t655, t2198, t5787, t5517, t1312, t13426, t18227, t2199, t2201, t2322, t27123, t27126, t28219, t4248, t4254, t5523, t651, t7732, t7889, t8307, t8321, t8325, t8327, t8393, t8411);
    (t31205, t31217, t31231, t31234, t31235, t31238, t31241, t31244, t31287, t31382, t31390, t31398)
}
