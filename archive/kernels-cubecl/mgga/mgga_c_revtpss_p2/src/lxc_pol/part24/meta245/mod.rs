//! MGGA_C_REVTPSS lxc pol kernel — _part24_v4rho4_4 meta245 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1007;
use chunk1::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1008;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_meta245<F: Float>(t1558: F, t2811: F, t2482: F, t1531: F, t37: F, t1544: F, t2475: F, t124: F, t136: F, t243: F, t220: F, t10815: F, t1561: F, t10845: F, t4430: F, t853: F, t4353: F, t9794: F, t10760: F, t10890: F, t1549: F, t4416: F, t808: F, t10886: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t14598, t14613, t14648, t14671, t14686, t14712) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1007::<F>(t1558, t2811, t2482, t1531, t37, t1544, t2475, t124, t136, t243, t220, t10815, t1561);
        let (t14716, t14718, t14761, t14765, t14779, t14780) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1008::<F>(t10845, t4430, t1558, t853, t4353, t9794, t10760, t10890, t1549, t4416, t808, t10886);
    (t14598, t14613, t14648, t14671, t14686, t14712, t14716, t14718, t14761, t14765, t14779, t14780)
}
