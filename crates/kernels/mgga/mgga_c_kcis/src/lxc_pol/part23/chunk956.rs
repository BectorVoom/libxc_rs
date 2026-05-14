//! MGGA_C_KCIS lxc pol — lxc_pol part 23 (v4rho3sigma_5) CSE chunk 956/1177 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part23_v4rho3sigma_5_chunk956<F: Float>(t27544: F, t4294: F, t1468: F, t4298: F, t1395: F, t4303: F, t4307: F, t27512: F, t27515: F, t27518: F, t27522: F, t27524: F, t27527: F, t27530: F, t27533: F, t27535: F, t27537: F, t27539: F, t27541: F) -> (F, F, F, F, F) {
    let t27545 = t27544 * t4294;
    let t27547 = t1468 * t4298;
    let t27549 = t1395 * t4303;
    let t27551 = t1395 * t4307;
    let t27553 = t27512 / 16.0 - t27515 / 8.0 + t27518 / 12.0 + t27522 / 8.0 - t27524 / 12.0 - t27527 / 16.0 - t27530 / 72.0 + t27533 / 24.0 - t27535 / 128.0 + t27537 / 64.0 - t27539 / 48.0 - t27541 / 64.0 + t27545 / 48.0 + t27547 / 128.0 - t27549 / 288.0 - t27551 / 96.0;
    (t27545, t27547, t27549, t27551, t27553)
}
