//! MGGA_C_REVTPSS lxc pol kernel — _part23_v4rho4_3 meta388 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1734;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_meta388<F: Float>(t16868: F, t16712: F, t16892: F, t16708: F, t1179: F, t5155: F, t1719: F, t3383: F, t1749: F, t3520: F, t3495: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t17050, t17052, t17066, t17075, t17089, t17092, t17097, t17115, t17117, t17131, t17140, t17154) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1734::<F>(t16868, t16712, t16892, t16708, t1179, t5155, t1719, t3383, t1749, t3520, t3495);
    (t17050, t17052, t17066, t17075, t17089, t17092, t17097, t17115, t17117, t17131, t17140, t17154)
}
