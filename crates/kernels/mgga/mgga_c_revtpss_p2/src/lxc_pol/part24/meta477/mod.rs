//! MGGA_C_REVTPSS lxc pol kernel — _part24_v4rho4_4 meta477 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1462;
use chunk1::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1463;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_meta477<F: Float>(t1065: F, t372: F, t6299: F, t3115: F, t42793: F, t6272: F, t19675: F, t1025: F, t371: F, t6276: F, t676: F, t15749: F, t4858: F, t3205: F, t6337: F, t15731: F, t4879: F, t225: F, t64686: F, t366: F, t19566: F, t3090: F, t1086: F, t19462: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
        let (t66777, t67015, t67052, t67186, t67195) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1462::<F>(t1065, t372, t6299, t3115, t42793, t6272, t19675, t1025, t371, t6276, t676, t15749, t4858);
        let (t67206, t67473, t67501, t67502, t67528, t67551) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1463::<F>(t3205, t371, t6337, t676, t15731, t4879, t225, t64686, t366, t19566, t3090, t1086, t19462);
    (t66777, t67015, t67052, t67186, t67195, t67206, t67473, t67501, t67502, t67528, t67551)
}
