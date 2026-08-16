//! MGGA_C_REVTPSS lxc pol kernel — _part23_v4rho4_3 meta617 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2292;
use chunk1::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2293;
use chunk2::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2294;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_meta617(t1715: f64, t21093: f64, t1042: f64, t1774: f64, t5819: f64, t5268: f64, t6573: f64, t482: f64, t371: f64, t372: f64, t12988: f64, t17308: f64, t17362: f64, t17417: f64, t17525: f64, t1791: f64, t1797: f64, t20820: f64, t20974: f64, t21001: f64, t21063: f64, t3711: f64, t5293: f64, t5323: f64, t5327: f64, t5384: f64, t6611: f64, t6625: f64, t6631: f64, t6647: f64, t12610: f64, t16706: f64, t20283: f64, t20285: f64, t20287: f64, t24230: f64, t24234: f64, t24238: f64, t24242: f64, t24246: f64, t24250: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t24604, t24605, t24610, t24611, t24612, t24616) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2292(t1715, t21093, t1042, t1774, t5819, t5268, t6573);
        let (t24617, t24619, t24622) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2293(t24616, t482, t371, t372, t12988, t17308, t17362, t17417, t17525, t1791, t1797, t20820, t20974, t21001, t21063, t24605, t24612, t3711, t5293, t5323, t5327, t5384, t6611, t6625, t6631, t6647);
        let t24633 = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2294(t12610, t16706, t20283, t20285, t20287, t24230, t24234, t24238, t24242, t24246, t24250);
    (t24604, t24605, t24610, t24611, t24612, t24616, t24617, t24619, t24622, t24633)
}
