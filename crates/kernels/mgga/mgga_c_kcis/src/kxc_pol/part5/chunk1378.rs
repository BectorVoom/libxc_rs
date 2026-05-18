//! MGGA_C_KCIS kxc pol — kxc_pol part 5 (v3rho3_2) CSE chunk 1378/1419 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_kxc_pol_part5_v3rho3_2_chunk1378<F: Float>(t1539: F, t7382: F, t22664: F, t4293: F, t6010: F, t4249: F, t7335: F, t2051: F, t5935: F, t1929: F, t4254: F, t6029: F) -> (F, F, F, F, F) {
    let t22672 = t7382 * t1539;
    let t22674 = t4293 * t22664;
    let t22675 = t6010 * t22674;
    let t22677 = t4249 * t7335;
    let t22679 = t2051 * t5935;
    let t22681 = t4254 * t1929;
    let t22682 = t22681 * t6029;
    (t22672, t22675, t22677, t22679, t22682)
}
