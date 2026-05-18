//! MGGA_C_KCIS lxc pol — lxc_pol part 26 (v4rho3sigma_8) CSE chunk 494/1397 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part26_v4rho3sigma_8_chunk494<F: Float>(t109: F, t494: F, t209: F, t617: F, t612: F, t1369: F, t25: F) -> (F, F, F) {
    let t4420 = t109 * t494;
    let t4422 = t209 * t4420 * t617;
    let t4424 = t612 * t4422 / F::new(864.0);
    let t4425 = t25 * t1369;
    (t4422, t4424, t4425)
}
