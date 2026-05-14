//! MGGA_C_KCISK kxc pol — kxc_pol part 3 (v3rho3_0) CSE chunk 879/938 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_kxc_pol_part3_v3rho3_0_chunk879<F: Float>(t13329: F, t492: F, t13331: F, t499: F, t498: F, t14150: F, t467: F, t500: F, t14400: F, t14402: F, t14514: F, t14516: F, t14519: F, t14522: F, t14525: F, t14529: F, t14532: F, t14536: F, t14538: F, t14541: F, t14543: F) -> (F, F, F) {
    let t14545 = t13329 * t492;
    let t14546 = t499 * t13331;
    let t14547 = t498 * t14546;
    let t14548 = t14545 * t14547;
    let t14550 = t14150 * t467;
    let t14551 = t14550 * t492;
    let t14552 = t14551 * t500;
    let t14554 = -3.0 / 16.0 * t14400 + 3.0 / 256.0 * t14402 + t14514 / 16.0 - t14516 / 192.0 + t14519 / 64.0 - t14522 / 24.0 + t14525 / 2.0 + t14529 / 24.0 - 3.0 / 128.0 * t14532 - t14536 / 192.0 - 2.0 / 3.0 * t14538 - t14541 / 3.0 + t14543 / 8.0 + 3.0 / 128.0 * t14548 - t14552 / 256.0;
    (t14548, t14552, t14554)
}
