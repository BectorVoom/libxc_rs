//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 21 (v4rho4_2) CSE chunk 2624/3221 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2624<F: Float>(t3809: F, t53945: F, t120: F, t16205: F, t12283: F, t16227: F, t1351: F, t5286: F, t12189: F, t5227: F, t16232: F, t3777: F) -> (F, F, F, F, F, F) {
    let t53946 = t53945 * t3809;
    let t53958 = t120 * t16205;
    let t53965 = t12283 * t16227;
    let t53973 = t5286 * t1351;
    let t53984 = t12189 * t5227;
    let t53990 = t3777 * t16232;
    (t53946, t53958, t53965, t53973, t53984, t53990)
}
