//! MGGA_C_REVTPSS lxc pol kernel — _part42_v4rho3tau_5 meta273 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_chunk1022;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_meta273(t1386: f64, t820: f64, t844: f64, t2482: f64, t596: f64, t4021: f64, t1384: f64, t235: f64, t4003: f64, t543: f64, t27: f64, t4000: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
        let (t9962, t9976, t9977, t9990, t9991, t9994, t10001) = mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_chunk1022(t1386, t820, t844, t2482, t596, t4021, t1384, t235, t4003, t543, t27, t4000);
    (t9962, t9976, t9977, t9990, t9991, t9994, t10001)
}
