//! MGGA_C_REVTPSS lxc pol kernel — _part21_v4rho4_1 meta627 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2390;
use chunk1::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2391;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_meta627<F: Float>(t10705: F, t10716: F, t10697: F, t136: F, t10627: F, t221: F, t2674: F, t2452: F, t9720: F, t225: F, t268: F, t2665: F, t10868: F, t240: F, t10871: F, t2661: F, t40479: F, t10726: F, t2723: F, t10638: F, t231: F, t243: F, t2662: F, t10722: F, t2656: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t40681, t40683, t40686, t40688, t40689, t40690, t40691) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2390::<F>(t10705, t10716, t10697, t136, t10627, t221, t2674, t2452, t9720, t225, t268, t2665);
        let (t40693, t40696, t40700, t40705, t40707) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2391::<F>(t10868, t240, t10871, t2661, t40479, t10726, t2723, t10638, t231, t243, t2662, t10722, t2656);
    (t40681, t40683, t40686, t40688, t40689, t40690, t40691, t40693, t40696, t40700, t40705, t40707)
}
