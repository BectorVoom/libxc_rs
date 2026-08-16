//! MGGA_C_REVTPSS lxc pol kernel — _part24_v4rho4_4 meta294 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1077;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_meta294(t1770: f64, t1811: f64, t3172: f64, t6618: f64, t3711: f64, t6634: f64, t3610: f64, t5265: f64, t5293: f64, t3153: f64, t6628: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
        let (t20756, t20783, t20784, t20786, t20787, t20789, t20795) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1077(t1770, t1811, t3172, t6618, t3711, t6634, t3610, t5265, t5293, t3153, t6628);
    (t20756, t20783, t20784, t20786, t20787, t20789, t20795)
}
