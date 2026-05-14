//! MGGA_C_KCIS lxc pol — lxc_pol part 23 (v4rho3sigma_5) CSE chunk 1079/1177 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part23_v4rho3sigma_5_chunk1079<F: Float>(t17449: F, t491: F, t7949: F, t3734: F, t5929: F, t11825: F, t27543: F, t17464: F, t17402: F, t7952: F, t8196: F, t94754: F, t1468: F, t17501: F, t27514: F, t5919: F) -> (F, F, F, F, F, F, F) {
    let t97701 = t17449 * t491;
    let t97702 = t97701 * t7949;
    let t97704 = t3734 * t5929;
    let t97706 = t11825 * t27543;
    let t97707 = t97706 * t17464;
    let t97709 = t7952 * t17402;
    let t97711 = t94754 * t8196;
    let t97713 = t1468 * t17501;
    let t97715 = t27514 * t5919;
    (t97702, t97704, t97707, t97709, t97711, t97713, t97715)
}
