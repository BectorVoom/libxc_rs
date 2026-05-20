//! MGGA_C_REVTPSS lxc pol — lxc_pol part 26 (v4rho3sigma_1) CSE chunk 739/1225 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part26_v4rho3sigma_1_chunk739<F: Float>(t9484: F, t9543: F, t520: F, t512: F, t1450: F, t4135: F, t177: F, t3850: F, t762: F, t749: F, t1353: F, t198: F, t4139: F, t566: F, t9399: F, t9400: F, t9405: F, t9407: F, t9409: F, t9412: F, t9415: F, t9421: F, t9423: F, t9427: F, t9430: F) -> (F, F, F, F, F) {
    let t9544 = t9484 + t9543;
    let t9545 = t520 * t9544;
    let t9546 = t512 * t9545;
    let t9547 = t4135 * t1450;
    let t9551 = t3850 * t177;
    let t9552 = t9551 * t762;
    let t9553 = F::cast_from(0.17544670867903938621e1_f64) * t9552;
    let t9554 = t3850 * t749;
    let t9555 = t512 * t9554;
    let t9556 = F::new(3.0) * t9555;
    let t9557 = F::new(9.0) * t1353 * t4139 * t9547 + F::new(6.0) * t198 * t566 * t9400 - t9399 + t9405 + t9407 - t9409 + t9412 - t9415 + t9421 + t9423 - t9427 + t9430 + t9546 - t9553 + t9556;
    (t9544, t9546, t9553, t9556, t9557)
}
