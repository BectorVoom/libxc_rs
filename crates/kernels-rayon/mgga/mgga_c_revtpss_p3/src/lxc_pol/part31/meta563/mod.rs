//! MGGA_C_REVTPSS lxc pol kernel — _part31_v4rho3sigma_6 meta563 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1974;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_meta563(t231: f64, t61756: f64, t1544: f64, t2411: f64, t22461: f64, t4147: f64, t6861: f64, t9994: f64, t1398: f64, t221: f64, t22274: f64, t22279: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
        let (t62695, t63185, t73407, t73820, t73842, t74419, t74423) = mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1974(t231, t61756, t1544, t2411, t22461, t4147, t6861, t9994, t1398, t221, t22274, t22279);
    (t62695, t63185, t73407, t73820, t73842, t74419, t74423)
}
