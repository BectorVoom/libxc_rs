//! MGGA_C_TPSSLOC lxc pol kernel — _part21_v4rho4_2 meta325 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1693;
use chunk1::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1694;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_meta325<F: Float>(t1294: F, t9494: F, t2535: F, t3691: F, t1372: F, t3787: F, t215: F, t535: F, t9569: F, t1314: F, t2559: F, t1317: F, t795: F, t9580: F, t3749: F, t9577: F, t3726: F, t3745: F, t2566: F) -> (F, F, F, F, F, F, F, F, F, F) {
        let (t12141, t12142, t12171, t12188, t12189) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1693::<F>(t1294, t9494, t2535, t3691, t1372, t3787, t215, t535, t9569, t1314, t2559);
        let (t12190, t12194, t12196, t12197, t12199) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1694::<F>(t12189, t1317, t535, t795, t9580, t3749, t9577, t3726, t3745, t1314, t2566);
    (t12141, t12142, t12171, t12188, t12189, t12190, t12194, t12196, t12197, t12199)
}
