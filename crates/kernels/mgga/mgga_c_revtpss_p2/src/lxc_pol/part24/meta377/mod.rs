//! MGGA_C_REVTPSS lxc pol kernel — _part24_v4rho4_4 meta377 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1270;
use chunk1::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1271;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_meta377<F: Float>(t476: F, t52: F, t475: F, t467: F, t1785: F, t6594: F, t12678: F, t16706: F, t20283: F, t20285: F, t20287: F, t24230: F, t24234: F, t24238: F, t24242: F, t24246: F, t24250: F, t459: F) -> (F, F, F, F, F, F, F) {
        let (t24677, t24679, t24680, t24681, t24684, t24697) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1270::<F>(t476, t52, t475, t467, t1785, t6594, t12678, t16706, t20283, t20285, t20287, t24230, t24234, t24238, t24242, t24246, t24250);
        let t24698 = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1271::<F>(t24697, t459);
    (t24677, t24679, t24680, t24681, t24684, t24697, t24698)
}
