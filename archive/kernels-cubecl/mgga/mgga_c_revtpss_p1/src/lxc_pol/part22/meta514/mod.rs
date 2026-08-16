//! MGGA_C_REVTPSS lxc pol kernel — _part22_v4rho4_2 meta514 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2279;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_meta514<F: Float>(t16807: F, t422: F, t12552: F, t1756: F, t12555: F, t3497: F, t1196: F, t16708: F, t16710: F, t16712: F, t12297: F, t12299: F, t12301: F, t12303: F, t12367: F, t16706: F, t16717: F, t16722: F, t16727: F, t16731: F, t16735: F, t16740: F, t16744: F, t16748: F) -> (F, F, F, F, F, F, F, F) {
        let (t16809, t16811, t16812, t16814, t16820, t16821, t16822, t16831) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2279::<F>(t16807, t422, t12552, t1756, t12555, t3497, t1196, t16708, t16710, t16712, t12297, t12299, t12301, t12303, t12367, t16706, t16717, t16722, t16727, t16731, t16735, t16740, t16744, t16748);
    (t16809, t16811, t16812, t16814, t16820, t16821, t16822, t16831)
}
