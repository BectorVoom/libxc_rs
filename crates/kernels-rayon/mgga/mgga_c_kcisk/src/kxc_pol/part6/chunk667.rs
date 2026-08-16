//! MGGA_C_KCISK kxc pol — kxc_pol part 6 (v3rho3_3) CSE chunk 667/1086 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcisk_kxc_pol_part6_v3rho3_3_chunk667(t9277: f64, t9290: f64, t2042: f64, t240: f64, t2666: f64, t5532: f64, t7656: f64, t802: f64, t8965: f64, t8967: f64, t8970: f64, t9095: f64, t9258: f64, t9262: f64) -> (f64, f64) {
    let t9291 = t9277 + t9290;
    let t9295 = t8965 - t8967 + t8970 - t9095 + t240 * (-t2042 * t9291 - 2.0_f64 * t2666 * t7656 + 2.0_f64 * t5532 * t9262 + t802 * t9258 - t8965 + t8967 - t8970 + t9095);
    (t9291, t9295)
}
