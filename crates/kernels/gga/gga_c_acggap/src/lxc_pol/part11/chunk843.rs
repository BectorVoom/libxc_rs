//! GGA_C_ACGGAP lxc pol — lxc_pol part 11 (v4rho3sigma_3) CSE chunk 843/1066 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part11_v4rho3sigma_3_chunk843<F: Float>(t31228: F, t7670: F, t1998: F, t3811: F, t30120: F, t7415: F, t1988: F, t7523: F, t7528: F, t7799: F, t1089: F, t12610: F, t2079: F, t2080: F, t1967: F, t7767: F) -> (F, F, F, F, F, F, F) {
    let t31229 = t31228 * t7670;
    let t31230 = 0.64311027177104605458e-3 * t31229;
    let t31231 = t1998 * t3811;
    let t31237 = t30120 * t7415;
    let t31239 = t1988 * t7523;
    let t31241 = t7799 * t7528;
    let t31245 = t2079 * t1089 * t12610 * t2080;
    let t31247 = t1967 * t7767;
    (t31230, t31231, t31237, t31239, t31241, t31245, t31247)
}
