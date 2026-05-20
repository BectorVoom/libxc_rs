//! MGGA_C_REVTPSS lxc pol kernel — _part23_v4rho4_3 meta617 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2292;
use chunk1::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2293;
use chunk2::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2294;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_meta617<F: Float>(t1715: F, t21093: F, t1042: F, t1774: F, t5819: F, t5268: F, t6573: F, t482: F, t371: F, t372: F, t12988: F, t17308: F, t17362: F, t17417: F, t17525: F, t1791: F, t1797: F, t20820: F, t20974: F, t21001: F, t21063: F, t3711: F, t5293: F, t5323: F, t5327: F, t5384: F, t6611: F, t6625: F, t6631: F, t6647: F, t12610: F, t16706: F, t20283: F, t20285: F, t20287: F, t24230: F, t24234: F, t24238: F, t24242: F, t24246: F, t24250: F) -> (F, F, F, F, F, F, F, F, F, F) {
        let (t24604, t24605, t24610, t24611, t24612, t24616) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2292::<F>(t1715, t21093, t1042, t1774, t5819, t5268, t6573);
        let (t24617, t24619, t24622) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2293::<F>(t24616, t482, t371, t372, t12988, t17308, t17362, t17417, t17525, t1791, t1797, t20820, t20974, t21001, t21063, t24605, t24612, t3711, t5293, t5323, t5327, t5384, t6611, t6625, t6631, t6647);
        let t24633 = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2294::<F>(t12610, t16706, t20283, t20285, t20287, t24230, t24234, t24238, t24242, t24246, t24250);
    (t24604, t24605, t24610, t24611, t24612, t24616, t24617, t24619, t24622, t24633)
}
