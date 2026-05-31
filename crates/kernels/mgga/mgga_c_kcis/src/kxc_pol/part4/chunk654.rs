//! MGGA_C_KCIS kxc pol — kxc_pol part 4 (v3rho3_1) CSE chunk 654/1420 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_kxc_pol_part4_v3rho3_1_chunk654<F: Float>(t3685: F, t3698: F, t1282: F, t1291: F, t187: F, t3324: F, t3327: F, t3333: F, t3482: F, t3662: F, t3664: F, t3669: F, t3670: F, t437: F) -> (F, F) {
    let t3699 = t3685 + t3698;
    let t3703 = t3324 - t3327 + t3333 - t3482 + t187 * (-t1282 * t3699 - F::cast_from(2.0_f64) * t1291 * t3664 + t3662 * t437 + F::cast_from(2.0_f64) * t3669 * t3670 - t3324 + t3327 - t3333 + t3482);
    (t3699, t3703)
}
