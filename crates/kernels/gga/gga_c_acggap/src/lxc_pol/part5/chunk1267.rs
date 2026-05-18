//! GGA_C_ACGGAP lxc pol — lxc_pol part 5 (v4rho4_2) CSE chunk 1267/1332 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part5_v4rho4_2_chunk1267<F: Float>(t1165: F, t3451: F, t4183: F, t5862: F, t3409: F, t5796: F, t12727: F, t1761: F, t1772: F, t368: F, t384: F, t398: F, t879: F) -> (F, F, F, F) {
    let t23396 = t3451 * t1165 * t5862 * t4183;
    let t23398 = t3409 * t5796;
    let t23405 = t12727 * t1761;
    let t23411 = t384 * t398 * t368 * t1772 * t879;
    (t23396, t23398, t23405, t23411)
}
