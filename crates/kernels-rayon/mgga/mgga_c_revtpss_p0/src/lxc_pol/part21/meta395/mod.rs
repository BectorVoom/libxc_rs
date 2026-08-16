//! MGGA_C_REVTPSS lxc pol kernel — _part21_v4rho4_1 meta395 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1849;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_meta395(t3712: f64, t372: f64, t3630: f64, t12705: f64, t5341: f64, t3720: f64, t5333: f64, t1263: f64, t675: f64) -> (f64, f64, f64, f64, f64, f64) {
        let (t12868, t12871, t12872, t12875, t12876, t12879) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1849(t3712, t372, t3630, t12705, t5341, t3720, t5333, t1263, t675);
    (t12868, t12871, t12872, t12875, t12876, t12879)
}
