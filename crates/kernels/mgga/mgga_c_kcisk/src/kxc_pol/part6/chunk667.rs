//! MGGA_C_KCISK kxc pol — kxc_pol part 6 (v3rho3_3) CSE chunk 667/1086 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_kxc_pol_part6_v3rho3_3_chunk667<F: Float>(t9277: F, t9290: F, t2042: F, t240: F, t2666: F, t5532: F, t7656: F, t802: F, t8965: F, t8967: F, t8970: F, t9095: F, t9258: F, t9262: F) -> (F, F) {
    let t9291 = t9277 + t9290;
    let t9295 = t8965 - t8967 + t8970 - t9095 + t240 * (-t2042 * t9291 - F::new(2.0) * t2666 * t7656 + F::new(2.0) * t5532 * t9262 + t802 * t9258 - t8965 + t8967 - t8970 + t9095);
    (t9291, t9295)
}
