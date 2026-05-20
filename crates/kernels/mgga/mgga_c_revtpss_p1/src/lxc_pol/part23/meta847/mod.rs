//! MGGA_C_REVTPSS lxc pol kernel — _part23_v4rho4_3 meta847 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2728;
use chunk1::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2729;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_meta847<F: Float>(t12916: F, t20837: F, t5331: F, t12910: F, t21003: F, t12809: F, t21029: F, t21177: F, t3678: F, t17303: F, t5327: F, t11249: F, t1248: F, t1284: F, t20849: F, t3624: F, t12772: F, t17729: F, t21036: F, t3625: F, t44250: F, t6639: F, t17423: F, t21049: F, t21439: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
        let (t70685, t70689, t70733, t70756, t70758, t70794) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2728::<F>(t12916, t20837, t5331, t12910, t21003, t12809, t21029, t21177, t3678, t17303, t5327, t11249, t1248);
        let (t70800, t70806, t70809, t70811, t70819) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2729::<F>(t1284, t20849, t3624, t12772, t17729, t21036, t3625, t44250, t6639, t17423, t21049, t21439);
    (t70685, t70689, t70733, t70756, t70758, t70794, t70800, t70806, t70809, t70811, t70819)
}
