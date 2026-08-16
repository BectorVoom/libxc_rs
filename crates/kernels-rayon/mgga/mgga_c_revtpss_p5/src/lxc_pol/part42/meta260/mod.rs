//! MGGA_C_REVTPSS lxc pol kernel — _part42_v4rho3tau_5 meta260 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_chunk994;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_meta260(t1317: f64, t3853: f64, t1320: f64, t4029: f64, t1333: f64, t3863: f64, t27: f64, t583: f64, t521: f64, t19: f64, t596: f64, t182: f64, t2490: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
        let (t9395, t9398, t9406, t9408, t9411, t9415, t9417) = mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_chunk994(t1317, t3853, t1320, t4029, t1333, t3863, t27, t583, t521, t19, t596, t182, t2490);
    (t9395, t9398, t9406, t9408, t9411, t9415, t9417)
}
