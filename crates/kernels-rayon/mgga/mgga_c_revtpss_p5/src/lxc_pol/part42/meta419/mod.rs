//! MGGA_C_REVTPSS lxc pol kernel — _part42_v4rho3tau_5 meta419 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_chunk1478;
use chunk1::mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_chunk1479;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_meta419(t31398: f64, t31461: f64, t3: f64, t2198: f64, t670: f64, t1518: f64, t31234: f64, t4292: f64, t8342: f64, t116: f64, t8406: f64, t117: f64, t31451: f64, param_d: f64, t1459: f64, t1461: f64, t1916: f64, t1918: f64, t2207: f64, t2209: f64, t572: f64, t573: f64, t5795: f64, t5802: f64, t5805: f64, t8336: f64, t8343: f64, t8346: f64, t8421: f64, t8427: f64, t8430: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t31463, t31464, t31475, t31493, t31494, t31497, t31500, t31505, t31506, t31509) = mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_chunk1478(t31398, t31461, t3, t2198, t670, t1518, t31234, t4292, t8342, t116, t8406, t117, t31451, param_d);
        let t31512 = mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_chunk1479(t1459, t1461, t1916, t1918, t2207, t2209, t31475, t31494, t31497, t31500, t31506, t31509, t572, t573, t5795, t5802, t5805, t8336, t8343, t8346, t8421, t8427, t8430);
    (t31463, t31464, t31475, t31493, t31494, t31497, t31500, t31505, t31506, t31509, t31512)
}
