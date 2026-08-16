//! MGGA_C_REVTPSS lxc pol kernel — _part23_v4rho4_3 meta329 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1627;
use chunk1::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1628;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_meta329<F: Float>(t13999: F, t5677: F, t13847: F, t13848: F, t1399: F, t9816: F, t2713: F, t3964: F, t5617: F, t5686: F, t9744: F, t221: F, t4019: F, t5659: F, t4018: F, t3989: F, t5629: F, t3930: F, t5661: F, t5665: F, t9976: F, t1412: F, t1882: F, t3938: F, t3992: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t14001, t14005, t14007, t14013, t14024, t14036) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1627::<F>(t13999, t5677, t13847, t13848, t1399, t9816, t2713, t3964, t5617, t5686, t9744, t221, t4019, t5659);
        let (t14038, t14040, t14042, t14043, t14045, t14047) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1628::<F>(t14036, t4018, t3989, t5629, t3930, t5661, t5665, t9976, t1412, t1882, t3938, t3992);
    (t14001, t14005, t14007, t14013, t14024, t14036, t14038, t14040, t14042, t14043, t14045, t14047)
}
