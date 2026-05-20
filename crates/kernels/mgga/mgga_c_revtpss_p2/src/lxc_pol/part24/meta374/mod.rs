//! MGGA_C_REVTPSS lxc pol kernel — _part24_v4rho4_4 meta374 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1266;
use chunk1::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1267;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_meta374<F: Float>(t24616: F, t482: F, t371: F, t372: F, t12988: F, t17308: F, t17362: F, t17417: F, t17525: F, t1791: F, t1797: F, t20820: F, t20974: F, t21001: F, t21063: F, t24605: F, t24612: F, t3711: F, t5293: F, t5323: F, t5327: F, t5384: F, t6611: F, t6625: F, t6631: F, t6647: F, t12610: F, t16706: F, t20283: F, t20285: F, t20287: F, t24230: F, t24234: F, t24238: F, t24242: F, t24246: F, t24250: F) -> (F, F, F, F) {
        let (t24617, t24619, t24622) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1266::<F>(t24616, t482, t371, t372, t12988, t17308, t17362, t17417, t17525, t1791, t1797, t20820, t20974, t21001, t21063, t24605, t24612, t3711, t5293, t5323, t5327, t5384, t6611, t6625, t6631, t6647);
        let t24633 = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1267::<F>(t12610, t16706, t20283, t20285, t20287, t24230, t24234, t24238, t24242, t24246, t24250);
    (t24617, t24619, t24622, t24633)
}
