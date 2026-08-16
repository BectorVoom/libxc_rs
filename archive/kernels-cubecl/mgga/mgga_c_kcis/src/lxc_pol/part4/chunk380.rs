//! MGGA_C_KCIS lxc pol — lxc_pol part 4 (v3rho3_1) CSE chunk 380/1420 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part4_v3rho3_1_chunk380<F: Float>(t1307: F, t1430: F, t1060: F, t323: F, t526: F, t251: F, t461: F) -> (F, F, F) {
    let t1431 = t1430 * t1307;
    let t1436 = F::cast_from(0.7925e-3_f64) * t323 * t1060 * t526;
    let t1437 = t251 * t461;
    (t1431, t1436, t1437)
}
