//! GGA_C_ACGGAP lxc pol — lxc_pol part 13 (v4rho3sigma_5) CSE chunk 930/1213 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part13_v4rho3sigma_5_chunk930<F: Float>(t3806: F, t7741: F, t3055: F, t597: F, t7670: F, t1998: F, t3811: F, t30120: F, t7415: F, t1988: F, t7523: F, t7528: F, t7799: F) -> (F, F, F, F, F, F, F) {
    let t31226 = t7741 * t3806;
    let t31227 = F::new(0.25724410870841842183e-2) * t31226;
    let t31228 = t3055 * t597;
    let t31229 = t31228 * t7670;
    let t31230 = F::new(0.64311027177104605458e-3) * t31229;
    let t31231 = t1998 * t3811;
    let t31237 = t30120 * t7415;
    let t31239 = t1988 * t7523;
    let t31241 = t7799 * t7528;
    (t31227, t31228, t31230, t31231, t31237, t31239, t31241)
}
