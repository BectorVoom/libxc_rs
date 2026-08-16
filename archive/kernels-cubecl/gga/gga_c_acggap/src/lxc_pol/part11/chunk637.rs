//! GGA_C_ACGGAP lxc pol — lxc_pol part 11 (v4rho3sigma_3) CSE chunk 637/1213 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part11_v4rho3sigma_3_chunk637<F: Float>(t1181: F, t4982: F, t1165: F, t4210: F, t540: F, t1163: F, t3169: F, t1005: F, t1423: F, t3765: F, t527: F, t398: F, t525: F, t966: F) -> (F, F, F, F, F, F, F) {
    let t4983 = t1181 * t4982;
    let t4987 = t1165 * t540 * t4210;
    let t4989 = F::cast_from(0.85748036236139473944e-3_f64) * t1163 * t4987;
    let t4991 = t1181 * t540 * t3169;
    let t4994 = t1005 * t1423;
    let t4996 = t3765 * t527;
    let t4999 = t398 * t966 * t525;
    (t4983, t4987, t4989, t4991, t4994, t4996, t4999)
}
