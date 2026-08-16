//! MGGA_C_REVTPSS lxc pol kernel — _part40_v4rho3tau_3 meta76 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part40_v4rho3tau_3_chunk457;
use chunk1::mgga_c_revtpss_lxc_pol_part40_v4rho3tau_3_chunk458;
use chunk2::mgga_c_revtpss_lxc_pol_part40_v4rho3tau_3_chunk459;
use chunk3::mgga_c_revtpss_lxc_pol_part40_v4rho3tau_3_chunk460;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part40_v4rho3tau_3_meta76<F: Float>(t1469: F, t60: F, t1474: F, t1480: F, t44: F, t56: F, t61: F, t626: F, t38: F, t633: F, t637: F, t77: F, t1471: F, t71: F, t85: F, t5: F, t1466: F, t603: F, t91: F, t117: F, t1468: F, t100: F, t55: F, tau1: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t1486, t1487, t1490, t1491, t1494) = mgga_c_revtpss_lxc_pol_part40_v4rho3tau_3_chunk457::<F>(t1469, t60, t1474, t1480, t44, t56, t61, t626, t38, t633, t637, t77);
        let t1497 = mgga_c_revtpss_lxc_pol_part40_v4rho3tau_3_chunk458::<F>(t1471, t1487, t1494, t71, t85);
        let t1501 = mgga_c_revtpss_lxc_pol_part40_v4rho3tau_3_chunk459::<F>(t5, t1466, t1497, t603, t91);
        let (t1502, t1504, t1505, t1507, t1509) = mgga_c_revtpss_lxc_pol_part40_v4rho3tau_3_chunk460::<F>(t117, t1501, t1468, t100, t55, tau1);
    (t1486, t1487, t1490, t1491, t1494, t1497, t1501, t1502, t1504, t1505, t1507, t1509)
}
