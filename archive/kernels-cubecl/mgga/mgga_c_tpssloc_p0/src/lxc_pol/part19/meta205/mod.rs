//! MGGA_C_TPSSLOC lxc pol kernel — _part19_v4rho4_0 meta205 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk877;
use chunk1::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk878;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_meta205<F: Float>(t3121: F, t884: F, t3071: F, t1023: F, t2780: F, t3036: F, t67: F, t3067: F, t3186: F, t3132: F, t3062: F, t820: F) -> (F, F, F, F, F, F, F, F, F, F) {
        let (t10393, t10394, t10397, t10398, t10401, t10402, t10403) = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk877::<F>(t3121, t884, t3071, t1023, t2780, t3036, t67, t3067, t3186);
        let (t10404, t10405, t10408) = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk878::<F>(t3132, t884, t3071, t3062, t820);
    (t10393, t10394, t10397, t10398, t10401, t10402, t10403, t10404, t10405, t10408)
}
