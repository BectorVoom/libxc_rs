//! MGGA_C_KCIS lxc pol — lxc_pol part 26 (v4rho3sigma_8) CSE chunk 832/1397 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part26_v4rho3sigma_8_chunk832<F: Float>(t16144: F, t5564: F, t659: F, t16050: F, t16048: F, t127: F, t368: F, t3751: F, t1477: F, t3754: F, t1482: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
    let t16145 = F::new(0.21908444444444444444e0) * t16144;
    let t16146 = t659 * t5564;
    let t16156 = F::new(0.39862222222222222222e0) * t16050;
    let t16183 = F::new(4.0) / F::new(27.0) * t16048;
    let t16184 = F::new(4.0) / F::new(9.0) * t16050;
    let t16232 = F::new(0.41203703703703703704e-2) * t16048;
    let t16233 = F::new(0.12361111111111111111e-1) * t16050;
    let t16292 = F::new(0.22076e0) * t16144;
    let t16301 = F::new(0.13418888888888888889e0) * t16048;
    let t16353 = t127 * t368 * t3751;
    let t16354 = t1477 * t3754;
    let t16359 = t1482 * t3754;
    (t16145, t16146, t16156, t16183, t16184, t16232, t16233, t16292, t16301, t16353, t16354, t16359)
}
