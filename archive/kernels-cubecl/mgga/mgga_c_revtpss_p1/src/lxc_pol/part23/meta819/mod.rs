//! MGGA_C_REVTPSS lxc pol kernel — _part23_v4rho4_3 meta819 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2667;
use chunk1::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2668;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_meta819<F: Float>(t11277: F, t19826: F, t16163: F, t4879: F, t1063: F, t19681: F, t3172: F, t11710: F, t19625: F, t4899: F, t19687: F, t15772: F, t4834: F, t1065: F, t19380: F, t1062: F, t19463: F, t19730: F, t3091: F, t20050: F, t3188: F, t20054: F, t18946: F, t247: F, t3109: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t65618, t65627, t65630, t65637, t65650, t65689) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2667::<F>(t11277, t19826, t16163, t4879, t1063, t19681, t3172, t11710, t19625, t4899, t19687, t15772, t4834);
        let (t65712, t65717, t65738, t65801, t65803, t65807) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2668::<F>(t1065, t19380, t1062, t19463, t11710, t19730, t3091, t20050, t3188, t20054, t1063, t18946, t247, t3109);
    (t65618, t65627, t65630, t65637, t65650, t65689, t65712, t65717, t65738, t65801, t65803, t65807)
}
