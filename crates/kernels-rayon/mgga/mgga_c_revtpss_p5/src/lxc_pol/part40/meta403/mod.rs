//! MGGA_C_REVTPSS lxc pol kernel — _part40_v4rho3tau_3 meta403 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part40_v4rho3tau_3_chunk1479;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part40_v4rho3tau_3_meta403(t31157: f64, t569: f64, t1453: f64, t8320: f64, t2198: f64, t4151: f64, t3813: f64, t508: f64, t1310: f64, t10416: f64, t1312: f64, t13435: f64, t13440: f64, t18163: f64, t2199: f64, t2201: f64, t2322: f64, t4254: f64, t5523: f64, t651: f64, t8307: f64, t8321: f64, t8325: f64, t8327: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
        let (t31158, t31161, t31164, t31169, t31172, t31201, t31204) = mgga_c_revtpss_lxc_pol_part40_v4rho3tau_3_chunk1479(t31157, t569, t1453, t8320, t2198, t4151, t3813, t508, t1310, t10416, t1312, t13435, t13440, t18163, t2199, t2201, t2322, t4254, t5523, t651, t8307, t8321, t8325, t8327);
    (t31158, t31161, t31164, t31169, t31172, t31201, t31204)
}
