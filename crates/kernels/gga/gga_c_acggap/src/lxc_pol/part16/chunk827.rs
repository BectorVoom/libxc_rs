//! GGA_C_ACGGAP lxc pol — lxc_pol part 16 (v4rho3sigma_8) CSE chunk 827/1080 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part16_v4rho3sigma_8_chunk827<F: Float>(t2087: F, t7780: F, t3806: F, t7741: F, t3055: F, t597: F, t7670: F, t1998: F, t3811: F, t7528: F, t7799: F, t2117: F, t980: F, t409: F, t2029: F, t7599: F) -> (F, F, F, F, F, F, F, F, F) {
    let t31224 = t7780 * t2087;
    let t31226 = t7741 * t3806;
    let t31227 = 0.25724410870841842183e-2 * t31226;
    let t31228 = t3055 * t597;
    let t31229 = t31228 * t7670;
    let t31230 = 0.64311027177104605458e-3 * t31229;
    let t31231 = t1998 * t3811;
    let t31241 = t7799 * t7528;
    let t31253 = t980 * t2117;
    let t31254 = t31253 * t409;
    let t31258 = t7599 * t2029;
    (t31224, t31227, t31228, t31230, t31231, t31241, t31253, t31254, t31258)
}
