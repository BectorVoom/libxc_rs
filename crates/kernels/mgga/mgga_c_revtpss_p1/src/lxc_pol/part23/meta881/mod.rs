//! MGGA_C_REVTPSS lxc pol kernel — _part23_v4rho4_3 meta881 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2790;
use chunk1::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2791;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_meta881<F: Float>(t2782: F, t4086: F, t543: F, t74982: F, t10073: F, t22373: F, t10069: F, t22369: F, t14216: F, t14239: F, t14220: F, t48007: F, t1883: F, t5658: F, t4100: F, t73842: F, t22331: F, t2470: F, t4101: F, t48048: F, t5741: F) -> (F, F, F, F, F, F, F, F, F, F) {
        let (t74985, t74990, t74999, t75003, t75005) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2790::<F>(t2782, t4086, t543, t74982, t10073, t22373, t10069, t22369, t14216, t14239, t14220, t48007);
        let (t75014, t75018, t75021, t75024, t75026) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2791::<F>(t1883, t5658, t2782, t4100, t543, t73842, t22331, t2470, t4101, t48048, t5741, t10073, t22369);
    (t74985, t74990, t74999, t75003, t75005, t75014, t75018, t75021, t75024, t75026)
}
