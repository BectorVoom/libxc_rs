//! MGGA_C_RMGGAC lxc pol — lxc_pol part 12 (v4rho3sigma_3) CSE chunk 933/1088 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part12_v4rho3sigma_3_chunk933<F: Float>(t16156: F, t9051: F, t1182: F, t615: F, t1971: F, t209: F, t236: F, t7453: F, t1175: F, t1475: F, t36336: F, t36343: F, t9147: F) -> (F, F, F, F, F) {
    let t40062 = t16156 * t9051;
    let t40063 = F::new(0.19863479950205658386e-4) * t40062;
    let t40064 = t615 * t1182;
    let t40068 = t7453 * t1971 * t236 * t40064 * t209;
    let t40073 = t36336 * t1971 * t236 * t1475 * t1175;
    let t40075 = t36343 * t9147;
    (t40063, t40064, t40068, t40073, t40075)
}
