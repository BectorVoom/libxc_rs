//! MGGA_C_REVTPSS lxc pol kernel — _part24_v4rho4_4 meta247 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1010;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_meta247<F: Float>(t2689: F, t4372: F, t4354: F, t9775: F, t10722: F, t1565: F, t10868: F, t241: F, t820: F, t2719: F, t844: F, t2482: F, t814: F) -> (F, F, F, F, F, F) {
        let (t14846, t14850, t14866, t14894, t14923, t14931) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1010::<F>(t2689, t4372, t4354, t9775, t10722, t1565, t10868, t241, t820, t2719, t844, t2482, t814);
    (t14846, t14850, t14866, t14894, t14923, t14931)
}
