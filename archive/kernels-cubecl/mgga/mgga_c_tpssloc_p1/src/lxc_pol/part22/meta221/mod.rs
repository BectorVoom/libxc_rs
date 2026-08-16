//! MGGA_C_TPSSLOC lxc pol kernel — _part22_v4rho4_3 meta221 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1268;
use chunk1::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1269;
use chunk2::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1270;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_meta221<F: Float>(t1458: F, t88: F, t1714: F, t460: F, t590: F, t60: F, t93: F, t101: F, t584: F, t16: F, t2: F, t591: F, t9: F, t21: F, t587: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
        let (t7676, t8034, t8705, t9108, t9174, t9211, t9212) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1268::<F>(t1458, t88, t1714, t460, t590, t60, t93, t101, t584, t16, t2);
        let (t9213, t9214) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1269::<F>(t9212, t591, t9);
        let (t9215, t9216) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1270::<F>(t9214, t21, t587);
    (t7676, t8034, t8705, t9108, t9174, t9211, t9212, t9213, t9214, t9215, t9216)
}
