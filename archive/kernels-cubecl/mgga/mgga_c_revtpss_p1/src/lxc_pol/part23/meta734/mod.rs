//! MGGA_C_REVTPSS lxc pol kernel — _part23_v4rho4_3 meta734 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2506;
use chunk1::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2507;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_meta734<F: Float>(t14869: F, t9775: F, t10899: F, t136: F, t216: F, t14786: F, t231: F, t40834: F, t854: F, t14833: F, t236: F, t2453: F, t9794: F, t14724: F, t10722: F, t4435: F, t10716: F, t14757: F, t10868: F, t2482: F, t814: F, t10845: F, t14732: F, t4423: F, t853: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
        let (t50443, t50446, t50451, t50454, t50457) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2506::<F>(t14869, t9775, t10899, t136, t216, t14786, t231, t40834, t854, t14833, t236, t2453, t9794);
        let (t50505, t50524, t50532, t50570, t50582, t50583) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2507::<F>(t14724, t9775, t10722, t4435, t10716, t14757, t10868, t2482, t814, t10845, t14732, t4423, t853);
    (t50443, t50446, t50451, t50454, t50457, t50505, t50524, t50532, t50570, t50582, t50583)
}
