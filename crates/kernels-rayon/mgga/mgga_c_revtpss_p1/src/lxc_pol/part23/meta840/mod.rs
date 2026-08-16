//! MGGA_C_REVTPSS lxc pol kernel — _part23_v4rho4_3 meta840 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2714;
use chunk1::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2715;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_meta840(t17217: f64, t17505: f64, t1032: f64, t1246: f64, t21333: f64, t17720: f64, t5391: f64, t11262: f64, t3610: f64, t6634: f64, t17569: f64, t5326: f64, t5390: f64, t17361: f64, t5293: f64, t1261: f64, t20863: f64, t3172: f64, t20973: f64, t3647: f64, t21242: f64, t3636: f64, t17306: f64, t17728: f64, t489: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t69947, t69958, t69961, t69964, t69966, t69968) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2714(t17217, t17505, t1032, t1246, t21333, t17720, t5391, t11262, t3610, t6634, t17569, t5326, t5390);
        let (t69971, t69984, t70006, t70008, t70014) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2715(t17361, t5293, t1261, t20863, t3172, t20973, t3647, t21242, t3636, t17306, t17728, t489);
    (t69947, t69958, t69961, t69964, t69966, t69968, t69971, t69984, t70006, t70008, t70014)
}
