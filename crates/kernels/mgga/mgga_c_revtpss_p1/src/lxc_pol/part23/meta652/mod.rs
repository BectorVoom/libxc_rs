//! MGGA_C_REVTPSS lxc pol kernel — _part23_v4rho4_3 meta652 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2380;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_meta652<F: Float>(t245: F, t40672: F, t10697: F, t136: F, t2452: F, t9720: F, t225: F, t268: F, t2665: F, t10868: F, t240: F, t2237: F, t2482: F, t849: F) -> (F, F, F, F, F, F, F, F) {
        let (t40673, t40683, t40688, t40689, t40690, t40691, t40693, t40710) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2380::<F>(t245, t40672, t10697, t136, t2452, t9720, t225, t268, t2665, t10868, t240, t2237, t2482, t849);
    (t40673, t40683, t40688, t40689, t40690, t40691, t40693, t40710)
}
