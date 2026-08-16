//! MGGA_C_REVTPSS lxc pol kernel — _part23_v4rho4_3 meta743 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2524;
use chunk1::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2525;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_meta743<F: Float>(t51483: F, t10069: F, t14588: F, t10518: F, t14606: F, t10073: F, t14504: F, t14575: F, t2435: F, t14568: F, t1568: F, t4503: F, t786: F, t40270: F, t4496: F, t136: F, t137: F, t14597: F, t2438: F, t2723: F, t49180: F, t836: F, t2457: F, t2710: F, t4469: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
        let (t51484, t51507, t51513, t51522, t51538, t51547, t51548) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2524::<F>(t51483, t10069, t14588, t10518, t14606, t10073, t14504, t14575, t2435, t14568, t1568, t4503);
        let (t51549, t51553, t51561, t51564) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2525::<F>(t51548, t786, t40270, t4496, t136, t137, t14597, t2438, t2723, t49180, t836, t2457, t2710, t4469);
    (t51484, t51507, t51513, t51522, t51538, t51547, t51548, t51549, t51553, t51561, t51564)
}
