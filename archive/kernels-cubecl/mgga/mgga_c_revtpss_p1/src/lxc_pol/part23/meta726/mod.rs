//! MGGA_C_REVTPSS lxc pol kernel — _part23_v4rho4_3 meta726 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2492;
use chunk1::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2493;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_meta726<F: Float>(t49186: F, t10142: F, t14113: F, t49180: F, t10136: F, t14239: F, t4101: F, t5740: F, t9288: F, t40270: F, t5737: F, t10073: F, t14207: F, t1398: F, t14141: F, t14143: F, t2434: F, t14155: F, t1432: F, t2470: F, t3999: F, t5710: F, t10069: F, t14225: F, t14114: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
        let (t49187, t49190, t49199, t49203, t49210, t49238) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2492::<F>(t49186, t10142, t14113, t49180, t10136, t14239, t4101, t5740, t9288, t40270, t5737, t10073, t14207);
        let (t49256, t49274, t49276, t49290, t49321) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2493::<F>(t1398, t14141, t14143, t2434, t14155, t1432, t2470, t3999, t5710, t10069, t14225, t10136, t14114);
    (t49187, t49190, t49199, t49203, t49210, t49238, t49256, t49274, t49276, t49290, t49321)
}
