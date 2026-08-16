//! MGGA_C_REVTPSS lxc pol kernel — _part42_v4rho3tau_5 meta229 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_chunk888;
use chunk1::mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_chunk889;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_meta229(t6141: f64, t935: f64, t915: f64, t2926: f64, t6109: f64, t2924: f64, t2930: f64, t4571: f64, t6094: f64, t6098: f64, t6102: f64, t1621: f64, t954: f64, t2950: f64, t2957: f64, t4620: f64, t6114: f64, t6121: f64, t6127: f64, t6129: f64, t6133: f64, t6136: f64, t6139: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t6142, t6144, t6145, t6147, t6152, t6157) = mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_chunk888(t6141, t935, t915, t2926, t6109, t2924, t2930, t4571, t6094, t6098, t6102, t1621);
        let (t6158, t6173) = mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_chunk889(t6157, t954, t2950, t2957, t4571, t4620, t6094, t6098, t6102, t6114, t6121, t6127, t6129, t6133, t6136, t6139);
    (t6142, t6144, t6145, t6147, t6152, t6157, t6158, t6173)
}
