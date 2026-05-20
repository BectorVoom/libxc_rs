//! MGGA_C_REVTPSS lxc pol kernel — _part23_v4rho4_3 meta372 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1701;
use chunk1::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1702;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_meta372<F: Float>(t1040: F, t15816: F, t1647: F, t3140: F, t3149: F, t11921: F, t247: F, t4757: F, t4837: F, t1659: F, t3105: F, t1062: F, t4797: F) -> (F, F, F, F, F, F, F) {
        let (t15817, t15822, t15823, t15827, t15829, t15830) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1701::<F>(t1040, t15816, t1647, t3140, t3149, t11921, t247, t4757, t4837, t1659, t3105);
        let t15850 = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1702::<F>(t1062, t4797);
    (t15817, t15822, t15823, t15827, t15829, t15830, t15850)
}
