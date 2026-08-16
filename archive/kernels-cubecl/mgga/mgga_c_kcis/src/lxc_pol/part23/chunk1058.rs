//! MGGA_C_KCIS lxc pol — lxc_pol part 23 (v4rho3sigma_5) CSE chunk 1058/1323 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part23_v4rho3sigma_5_chunk1058<F: Float>(t27544: F, t4294: F, t1468: F, t4298: F, t1395: F, t4303: F, t4307: F, t27512: F, t27515: F, t27518: F, t27522: F, t27524: F, t27527: F, t27530: F, t27533: F, t27535: F, t27537: F, t27539: F, t27541: F) -> (F, F, F, F, F) {
    let t27545 = t27544 * t4294;
    let t27547 = t1468 * t4298;
    let t27549 = t1395 * t4303;
    let t27551 = t1395 * t4307;
    let t27553 = t27512 / F::cast_from(16.0_f64) - t27515 / F::cast_from(8.0_f64) + t27518 / F::cast_from(12.0_f64) + t27522 / F::cast_from(8.0_f64) - t27524 / F::cast_from(12.0_f64) - t27527 / F::cast_from(16.0_f64) - t27530 / F::cast_from(72.0_f64) + t27533 / F::cast_from(24.0_f64) - t27535 / F::cast_from(128.0_f64) + t27537 / F::cast_from(64.0_f64) - t27539 / F::cast_from(48.0_f64) - t27541 / F::cast_from(64.0_f64) + t27545 / F::cast_from(48.0_f64) + t27547 / F::cast_from(128.0_f64) - t27549 / F::cast_from(288.0_f64) - t27551 / F::cast_from(96.0_f64);
    (t27545, t27547, t27549, t27551, t27553)
}
