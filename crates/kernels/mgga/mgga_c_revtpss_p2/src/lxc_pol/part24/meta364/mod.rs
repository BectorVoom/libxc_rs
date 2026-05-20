//! MGGA_C_REVTPSS lxc pol kernel — _part24_v4rho4_4 meta364 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1243;
use chunk1::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1244;
use chunk2::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1245;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_meta364<F: Float>(t1120: F, t24248: F, t128: F, t12367: F, t16706: F, t20283: F, t20285: F, t20287: F, t24230: F, t24234: F, t24238: F, t24242: F, t24246: F, t448: F, t300: F, t1733: F, t20629: F, t5063: F, t6471: F, t16840: F, t6474: F, t24220: F, t3435: F, t12248: F, t5071: F, t6449: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
        let (t24249, t24250) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1243::<F>(t1120, t24248, t128);
        let (t24252, t24253) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1244::<F>(t12367, t16706, t20283, t20285, t20287, t24230, t24234, t24238, t24242, t24246, t24250, t448);
        let (t24255, t24257, t24259, t24261, t24262, t24264, t24265) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1245::<F>(t24253, t300, t1733, t20629, t5063, t6471, t16840, t6474, t24220, t3435, t12248, t5071, t6449);
    (t24249, t24250, t24252, t24253, t24255, t24257, t24259, t24261, t24262, t24264, t24265)
}
