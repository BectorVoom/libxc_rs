//! GGA_C_FT97 lxc pol — lxc_pol part 21 (v4rho3sigma_6) CSE chunk 1253/1339 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part21_v4rho3sigma_6_chunk1253<F: Float>(t1349: F, t30033: F, t376: F, t23405: F, t30141: F, t1058: F, t6615: F, t30149: F, t5956: F, t61366: F, t104446: F, t104474: F, t104484: F, t1969: F, t24102: F, t26581: F, t26769: F, t26780: F, t26817: F, t26823: F, t28: F, t379: F, t4454: F, t5772: F, t6580: F, t6584: F, t6618: F, t9049: F) -> (F, F) {
    let t119318 = t1349 * t376 * t30033;
    let t119322 = t23405 * t30141;
    let t119330 = t6615 * t1058;
    let t119343 = t23405 * t30149;
    let t119345 = t61366 * t5956;
    let t119347 = -t119318 / 9.0 + t6580 * t26780 / 3.0 + t119322 / 27.0 - t104474 + t26581 * t6618 / 3.0 + t104484 + t1349 * t28 * t26769 * t1058 / 3.0 - t5772 * t1969 * t119330 * t379 / 9.0 - t5772 * t9049 * t24102 * t4454 / 27.0 - t104446 * t6584 / 9.0 - t26817 * t26823 / 9.0 + t119343 / 81.0 + 4.0 * t119345;
    (t119345, t119347)
}
