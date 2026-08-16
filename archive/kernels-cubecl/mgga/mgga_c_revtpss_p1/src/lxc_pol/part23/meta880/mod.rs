//! MGGA_C_REVTPSS lxc pol kernel — _part23_v4rho4_3 meta880 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2788;
use chunk1::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2789;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_meta880<F: Float>(t1882: F, t2482: F, t4104: F, t5767: F, t1892: F, t5658: F, t14230: F, t2782: F, t48083: F, t4086: F, t543: F, t10073: F, t22365: F, t14141: F, t14143: F, t676: F, t22252: F, t555: F, t1419: F, t6843: F, t14224: F, t14238: F, t6861: F) -> (F, F, F, F, F, F, F, F, F) {
        let (t74908, t74935, t74943, t74945) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2788::<F>(t1882, t2482, t4104, t5767, t1892, t5658, t14230, t2782, t48083, t4086, t543, t10073, t22365);
        let (t74949, t74965, t74973, t74979, t74982) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2789::<F>(t14141, t14143, t5658, t676, t22252, t555, t1419, t6843, t14224, t14238, t2782, t6861);
    (t74908, t74935, t74943, t74945, t74949, t74965, t74973, t74979, t74982)
}
