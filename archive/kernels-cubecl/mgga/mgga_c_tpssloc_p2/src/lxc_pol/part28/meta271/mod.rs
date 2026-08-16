//! MGGA_C_TPSSLOC lxc pol kernel — _part28_v4rho3sigma_4 meta271 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1157;
use chunk1::mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1158;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_meta271<F: Float>(t3: F, t7945: F, t1458: F, t2039: F, t1401: F, t3941: F, t5371: F, t577: F, t7230: F, t7801: F, t590: F, t60: F, t192: F, t533: F, t1390: F, t2094: F, t16: F, t2: F, t591: F, t9: F, t21: F, t587: F, t14: F, t598: F) -> (F, F, F, F, F, F, F, F, F, F) {
        let (t7946, t7956, t7961, t8705) = mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1157::<F>(t3, t7945, t1458, t2039, t1401, t3941, t5371, t577, t7230, t7801, t590, t60);
        let (t8944, t9016, t9212, t9214, t9216, t9218) = mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1158::<F>(t192, t533, t1390, t2094, t16, t2, t591, t9, t21, t587, t14, t598);
    (t7946, t7956, t7961, t8705, t8944, t9016, t9212, t9214, t9216, t9218)
}
