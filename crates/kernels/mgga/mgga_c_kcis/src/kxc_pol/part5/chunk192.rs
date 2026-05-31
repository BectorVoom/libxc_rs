//! MGGA_C_KCIS kxc pol — kxc_pol part 5 (v3rho3_2) CSE chunk 192/1419 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_kxc_pol_part5_v3rho3_2_chunk192<F: Float>(t578: F, t585: F, t574: F) -> (F, F, F) {
    let t586 = t578 * t585;
    let t588 = F::cast_from(1.0_f64) + t574 / F::cast_from(16.0_f64) - t586 / F::cast_from(256.0_f64);
    let t589 = F::cast_from(1.0_f64) / t588;
    (t586, t588, t589)
}
