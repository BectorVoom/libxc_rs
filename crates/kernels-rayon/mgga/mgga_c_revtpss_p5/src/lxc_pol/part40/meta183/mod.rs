//! MGGA_C_REVTPSS lxc pol kernel — _part40_v4rho3tau_3 meta183 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part40_v4rho3tau_3_chunk789;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part40_v4rho3tau_3_meta183(t1399: f64, t221: f64, t4019: f64, t4018: f64, t1317: f64, t1331: f64, t1333: f64, t2522: f64, t2562: f64, t2569: f64, t2579: f64, t2587: f64, t3852: f64, t3854: f64, t3871: f64, t3873: f64) -> (f64, f64, f64, f64, f64, f64) {
        let (t4021, t4022, t4024, t4025, t4027, t4028) = mgga_c_revtpss_lxc_pol_part40_v4rho3tau_3_chunk789(t1399, t221, t4019, t4018, t1317, t1331, t1333, t2522, t2562, t2569, t2579, t2587, t3852, t3854, t3871, t3873);
    (t4021, t4022, t4024, t4025, t4027, t4028)
}
