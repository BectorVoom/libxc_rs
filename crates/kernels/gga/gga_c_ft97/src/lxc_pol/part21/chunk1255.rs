//! GGA_C_FT97 lxc pol — lxc_pol part 21 (v4rho3sigma_6) CSE chunk 1255/1339 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part21_v4rho3sigma_6_chunk1255<F: Float>(t1349: F, t30289: F, t376: F, t30123: F, t17409: F, t5968: F, t104477: F, t104532: F, t104541: F, t104549: F, t104552: F, t104554: F, t104562: F, t107707: F, t16675: F, t26515: F, t26793: F, t30156: F, t5845: F, t6580: F) -> (F, F) {
    let t119387 = t1349 * t376 * t30289;
    let t119390 = t1349 * t376 * t30123;
    let t119403 = t17409 * t5968;
    let t119405 = -t119387 / 3.0 + t119390 / 9.0 + t104532 + t6580 * t26515 / 3.0 + t30156 * t5845 / 6.0 - 4.0 / 81.0 * t104541 + t104549 - t104552 + t104554 - 2.0 / 3.0 * t6580 * t26793 + 2.0 / 27.0 * t104562 + 4.0 / 27.0 * t104477 * t107707 * t16675 - 2.0 * t119403;
    (t119403, t119405)
}
