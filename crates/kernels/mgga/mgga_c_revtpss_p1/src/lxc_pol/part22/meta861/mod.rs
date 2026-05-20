//! MGGA_C_REVTPSS lxc pol kernel — _part22_v4rho4_2 meta861 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3011;
use chunk1::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3012;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_meta861<F: Float>(t14724: F, t9775: F, t1558: F, t2722: F, t10726: F, t2661: F, t2724: F, t4416: F, t4352: F, t10722: F, t4435: F, t14751: F, t2652: F, t14769: F, t10716: F, t14757: F, t14772: F, t221: F, t2674: F, t40683: F, t2645: F, t10868: F, t2482: F, t814: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
        let (t50504, t50511, t50518, t50522, t50524, t50526) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3011::<F>(t14724, t9775, t1558, t2722, t10726, t2661, t2724, t4416, t4352, t10722, t4435, t14751, t2652);
        let (t50529, t50531, t50540, t50560, t50570) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3012::<F>(t14769, t2652, t10716, t14757, t14772, t221, t2674, t40683, t1558, t2645, t10868, t2482, t814);
    (t50504, t50511, t50518, t50522, t50524, t50526, t50529, t50531, t50540, t50560, t50570)
}
