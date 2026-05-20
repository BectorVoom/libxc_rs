//! MGGA_C_REVTPSS lxc pol kernel — _part23_v4rho4_3 meta854 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2741;
use chunk1::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2742;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_meta854<F: Float>(t17290: F, t5362: F, t17435: F, t5327: F, t3655: F, t6595: F, t1256: F, t21313: F, t21316: F, t1261: F, t20272: F, t247: F, t3634: F, t12916: F, t20951: F, t5340: F, t17396: F, t17620: F, t17472: F, t5373: F, t1222: F, t17471: F, t20266: F, t17351: F, t20770: F, t56756: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
        let (t71740, t71742, t71744, t71749, t71751, t71827) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2741::<F>(t17290, t5362, t17435, t5327, t3655, t6595, t1256, t21313, t21316, t1261, t20272, t247, t3634);
        let (t71845, t71859, t71880, t71883, t71886) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2742::<F>(t12916, t20951, t5340, t17396, t17620, t17472, t5373, t1222, t17471, t20266, t17351, t20770, t56756);
    (t71740, t71742, t71744, t71749, t71751, t71827, t71845, t71859, t71880, t71883, t71886)
}
