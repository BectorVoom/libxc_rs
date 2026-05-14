//! MGGA_C_KCIS lxc pol — lxc_pol part 26 (v4rho3sigma_8) CSE chunk 1191/1243 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part26_v4rho3sigma_8_chunk1191<F: Float>(t576: F, t5905: F, t97800: F, t22259: F, t97793: F, t4122: F, t6012: F, t5916: F, t97767: F, t5913: F, t3734: F, t7305: F, t6029: F, t97804: F, t22354: F, t27544: F) -> (F, F, F, F, F, F, F, F) {
    let t102886 = t576 * t97800 * t5905;
    let t102889 = t97793 * t22259;
    let t102892 = t4122 * t97800 * t6012;
    let t102894 = t97767 * t5916;
    let t102896 = t97767 * t5913;
    let t102898 = t3734 * t7305;
    let t102900 = t97804 * t6029;
    let t102902 = t27544 * t22354;
    (t102886, t102889, t102892, t102894, t102896, t102898, t102900, t102902)
}
