//! MGGA_C_RMGGAC lxc pol — lxc_pol part 13 (v4rho3sigma_4) CSE chunk 807/988 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part13_v4rho3sigma_4_chunk807<F: Float>(t34724: F, t8626: F, t504: F, t8629: F, t8632: F, t16156: F, t9051: F, t1182: F, t615: F, t1971: F, t209: F, t236: F, t7453: F, t1175: F, t1475: F, t36336: F) -> (F, F, F, F, F, F) {
    let t40057 = t34724 * t8626;
    let t40060 = t504 * t8629 * t8632;
    let t40062 = t16156 * t9051;
    let t40064 = t615 * t1182;
    let t40068 = t7453 * t1971 * t236 * t40064 * t209;
    let t40073 = t36336 * t1971 * t236 * t1475 * t1175;
    (t40057, t40060, t40062, t40064, t40068, t40073)
}
