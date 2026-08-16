//! MGGA_C_TPSSLOC lxc pol kernel — _part19_v4rho4_0 meta282 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1047;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_meta282<F: Float>(t1314: F, t2559: F, t1317: F, t535: F, t795: F, t9580: F, t3749: F, t9577: F, t3726: F, t3745: F, t2566: F, t3741: F) -> (F, F, F, F, F, F, F) {
        let (t12189, t12190, t12194, t12196, t12197, t12199, t12200) = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1047::<F>(t1314, t2559, t1317, t535, t795, t9580, t3749, t9577, t3726, t3745, t2566, t3741);
    (t12189, t12190, t12194, t12196, t12197, t12199, t12200)
}
