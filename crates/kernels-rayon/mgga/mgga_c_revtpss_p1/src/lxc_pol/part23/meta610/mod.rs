//! MGGA_C_REVTPSS lxc pol kernel — _part23_v4rho4_3 meta610 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2274;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_meta610(t1139: f64, t24312: f64, t1132: f64, t1723: f64, t6442: f64, t12327: f64, t12331: f64, t12349: f64, t12352: f64, t24238: f64, t24242: f64, t24246: f64, t24250: f64, t24289: f64, t24292: f64, t24295: f64, t24298: f64) -> (f64, f64, f64, f64, f64, f64) {
        let (t24313, t24315, t24317, t24318, t24320, t24322) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2274(t1139, t24312, t1132, t1723, t6442, t12327, t12331, t12349, t12352, t24238, t24242, t24246, t24250, t24289, t24292, t24295, t24298);
    (t24313, t24315, t24317, t24318, t24320, t24322)
}
