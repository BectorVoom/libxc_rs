//! GGA_C_FT97 kxc pol — kxc_pol part 3 (v3rho3_2) CSE chunk 609/887 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_kxc_pol_part3_v3rho3_2_chunk609<F: Float>(t2644: F, t342: F, t630: F, t2680: F, t683: F, t191: F, t7640: F, t793: F, t89: F, t9733: F, t272: F, t9606: F, t274: F, t668: F, t505: F, t123: F, t805: F) -> (F, F, F, F, F, F, F) {
    let t10212 = t342 * t630 * t2644;
    let t10248 = t683 * t2680;
    let t10261 = t191 * t7640;
    let t10279 = t89 * t9733 * t793;
    let t10304 = 1.0 / t272 / t9606;
    let t10327 = t274 * t668;
    let t10328 = t10327 * t505;
    let t10339 = t123 / t805 / t9606;
    (t10212, t10248, t10261, t10279, t10304, t10328, t10339)
}
