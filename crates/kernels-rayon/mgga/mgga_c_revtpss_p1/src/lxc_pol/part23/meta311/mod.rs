//! MGGA_C_REVTPSS lxc pol kernel — _part23_v4rho4_3 meta311 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1591;
use chunk1::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1592;
use chunk2::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1593;
use chunk3::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1594;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_meta311(t1260: f64, t3666: f64, t12640: f64, t225: f64, t480: f64, t1236: f64, t371: f64, t676: f64, t1235: f64, t12627: f64, t1224: f64, t3362: f64, t1226: f64, t697: f64, t1222: f64, t12268: f64, t3698: f64, t3367: f64, t404: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let t12956 = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1591(t1260, t3666);
        let t12966 = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1592(t12640, t225);
        let (t12967, t12984, t12985, t12987) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1593(t12966, t480, t1236, t371, t676, t1235, t12627, t225);
        let (t12988, t13006, t13011, t13012, t13020, t13026) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1594(t12987, t480, t1224, t3362, t1226, t697, t1222, t12268, t3698, t3367, t404);
    (t12956, t12966, t12967, t12984, t12985, t12987, t12988, t13006, t13011, t13012, t13020, t13026)
}
