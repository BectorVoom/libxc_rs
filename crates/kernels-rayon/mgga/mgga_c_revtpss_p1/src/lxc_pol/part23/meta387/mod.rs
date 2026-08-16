//! MGGA_C_REVTPSS lxc pol kernel — _part23_v4rho4_3 meta387 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1733;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_meta387(t16710: f64, t16712: f64, t1737: f64, t3451: f64, t1160: f64, t5117: f64, t3476: f64) -> (f64, f64, f64, f64, f64) {
        let (t17010, t17011, t17023, t17026, t17032) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1733(t16710, t16712, t1737, t3451, t1160, t5117, t3476);
    (t17010, t17011, t17023, t17026, t17032)
}
