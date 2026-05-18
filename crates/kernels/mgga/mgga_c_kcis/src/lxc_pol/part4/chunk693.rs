//! MGGA_C_KCIS lxc pol — lxc_pol part 4 (v3rho3_1) CSE chunk 693/1420 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part4_v3rho3_1_chunk693<F: Float>(t1360: F, t1363: F, t110: F, t499: F, t493: F, t1369: F, t24: F) -> (F, F, F, F) {
    let t3964 = t1360 * t1363;
    let t3967 = t110 * t499;
    let t3969 = t493 * t3967 / F::new(432.0);
    let t3970 = t24 * t1369;
    (t3964, t3967, t3969, t3970)
}
