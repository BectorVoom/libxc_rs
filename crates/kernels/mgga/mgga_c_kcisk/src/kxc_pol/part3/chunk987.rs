//! MGGA_C_KCISK kxc pol — kxc_pol part 3 (v3rho3_0) CSE chunk 987/1063 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_kxc_pol_part3_v3rho3_0_chunk987<F: Float>(t14546: F, t498: F, t14545: F, t14150: F, t467: F, t492: F, t500: F, t14400: F, t14402: F, t14514: F, t14516: F, t14519: F, t14522: F, t14525: F, t14529: F, t14532: F, t14536: F, t14538: F, t14541: F, t14543: F) -> (F, F, F) {
    let t14547 = t498 * t14546;
    let t14548 = t14545 * t14547;
    let t14550 = t14150 * t467;
    let t14551 = t14550 * t492;
    let t14552 = t14551 * t500;
    let t14554 = -F::new(3.0) / F::new(16.0) * t14400 + F::new(3.0) / F::new(256.0) * t14402 + t14514 / F::new(16.0) - t14516 / F::new(192.0) + t14519 / F::new(64.0) - t14522 / F::new(24.0) + t14525 / F::new(2.0) + t14529 / F::new(24.0) - F::new(3.0) / F::new(128.0) * t14532 - t14536 / F::new(192.0) - F::new(2.0) / F::new(3.0) * t14538 - t14541 / F::new(3.0) + t14543 / F::new(8.0) + F::new(3.0) / F::new(128.0) * t14548 - t14552 / F::new(256.0);
    (t14548, t14552, t14554)
}
