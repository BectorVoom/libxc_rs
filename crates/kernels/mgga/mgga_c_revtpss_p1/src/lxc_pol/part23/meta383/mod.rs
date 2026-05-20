//! MGGA_C_REVTPSS lxc pol kernel — _part23_v4rho4_3 meta383 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1725;
use chunk1::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1726;
use chunk2::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1727;
use chunk3::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1728;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_meta383<F: Float>(t1695: F, t3268: F, t12230: F, t1732: F, t3495: F, t5180: F, t3302: F, t5332: F, t1716: F, t2435: F, t5048: F, t689: F, t5053: F, t5057: F) -> (F, F, F, F, F, F, F, F, F) {
        let (t16604, t16668, t16676, t16695, t16706) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1725::<F>(t1695, t3268, t12230, t1732, t3495, t5180, t3302, t5332, t1716, t2435);
        let t16708 = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1726::<F>(t5048, t689);
        let t16710 = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1727::<F>(t5053, t689);
        let (t16711, t16712) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1728::<F>(t16710, t5057, t689);
    (t16604, t16668, t16676, t16695, t16706, t16708, t16710, t16711, t16712)
}
