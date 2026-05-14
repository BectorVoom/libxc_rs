//! MGGA_C_REVTPSS lxc pol — lxc_pol part 4 (v3rho3_1) CSE chunk 973/1274 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part4_v3rho3_1_chunk973<F: Float>(t1058: F, t3191: F, t1021: F, t3201: F, t3231: F, t1054: F, t2434: F, t371: F, t373: F, t367: F, t3123: F, t3168: F, t3124: F, t3173: F, t1065: F, t675: F) -> (F, F, F, F, F, F, F, F) {
    let t11954 = t3191 * t1058;
    let t11956 = t1021 * t3201;
    let t11965 = t3231 * t1058;
    let t11967 = t1054 * t3201;
    let t11970 = t371 * t2434 * t373;
    let t11972 = 0.63517063878621832551e-4 * t367 * t11970;
    let t11977 = t3123 * t3168;
    let t11980 = t3124 * t3173;
    let t11986 = t675 * t1065;
    (t11954, t11956, t11965, t11967, t11972, t11977, t11980, t11986)
}
