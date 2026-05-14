//! GGA_C_FT97 kxc pol — kxc_pol part 3 (v3rho3_2) CSE chunk 597/887 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_kxc_pol_part3_v3rho3_2_chunk597<F: Float>(t363: F, t8965: F, t123: F, t532: F, t7911: F, t122: F, t29: F, t32: F, t23: F, t7368: F, t1642: F, t525: F, t1636: F, t559: F, t89: F, t10: F, t144: F, t3050: F) -> (F, F, F, F, F, F, F, F) {
    let t8966 = t8965 * t363;
    let t8977 = t123 / t532 / t7911;
    let t8991 = t122 * t122;
    let t8994 = t8991 / t32 / t29;
    let t9016 = t23 * t7368;
    let t9049 = t1642 * t525;
    let t9065 = t89 * t1636 * t559;
    let t9071 = t10 * t3050 * t144;
    (t8966, t8977, t8991, t8994, t9016, t9049, t9065, t9071)
}
