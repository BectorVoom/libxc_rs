//! MGGA_C_REVTPSS lxc pol kernel — _part21_v4rho4_1 meta551 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2230;
use chunk1::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2231;
use chunk2::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2232;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_meta551<F: Float>(t3667: F, t5362: F, t1789: F, t371: F, t676: F, t1235: F, t1769: F, t3565: F, t225: F, t480: F, t1803: F, t3650: F, t16708: F, t16710: F, t16712: F, t12297: F, t12299: F, t12301: F, t12303: F, t12678: F, t16706: F, t16717: F, t16722: F, t16727: F, t16731: F, t16735: F, t16740: F, t16744: F, t16748: F, t459: F) -> (F, F, F, F, F, F, F, F, F) {
        let (t17301, t17303, t17304, t17306) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2230::<F>(t3667, t5362, t1789, t371, t676, t1235, t1769, t3565);
        let (t17307, t17308, t17311, t17330) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2231::<F>(t17306, t225, t480, t1803, t3650, t16708, t16710, t16712, t12297, t12299, t12301, t12303, t12678, t16706, t16717, t16722, t16727, t16731, t16735, t16740, t16744, t16748);
        let t17331 = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2232::<F>(t17330, t459);
    (t17301, t17303, t17304, t17306, t17307, t17308, t17311, t17330, t17331)
}
