//! GGA_C_FT97 kxc pol — kxc_pol part 3 (v3rho3_2) CSE chunk 673/1032 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_kxc_pol_part3_v3rho3_2_chunk673<F: Float>(t1526: F, t2640: F, t9483: F, t2644: F, t342: F, t630: F, t2680: F, t683: F, t191: F, t7640: F, t793: F, t89: F, t9733: F) -> (F, F, F, F, F) {
    let t10209 = t1526 * t9483 * t2640;
    let t10212 = t342 * t630 * t2644;
    let t10248 = t683 * t2680;
    let t10261 = t191 * t7640;
    let t10279 = t89 * t9733 * t793;
    (t10209, t10212, t10248, t10261, t10279)
}
