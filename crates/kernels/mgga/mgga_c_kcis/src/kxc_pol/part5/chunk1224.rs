//! MGGA_C_KCIS kxc pol — kxc_pol part 5 (v3rho3_2) CSE chunk 1224/1260 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_kxc_pol_part5_v3rho3_2_chunk1224<F: Float>(t22674: F, t6010: F, t4249: F, t7335: F, t2051: F, t5935: F, t1929: F, t4254: F, t6029: F, t7283: F, t6002: F, t6034: F, t2066: F, t6020: F, t21955: F, t577: F) -> (F, F, F, F, F, F, F, F) {
    let t22675 = t6010 * t22674;
    let t22677 = t4249 * t7335;
    let t22679 = t2051 * t5935;
    let t22681 = t4254 * t1929;
    let t22682 = t22681 * t6029;
    let t22685 = t4249 * t7283;
    let t22687 = t6002 * t6034;
    let t22689 = t6020 * t2066;
    let t22691 = t21955 * t577;
    (t22675, t22677, t22679, t22682, t22685, t22687, t22689, t22691)
}
