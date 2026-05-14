//! GGA_C_ACGGAP lxc pol — lxc_pol part 13 (v4rho3sigma_5) CSE chunk 842/1066 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part13_v4rho3sigma_5_chunk842<F: Float>(t13287: F, t2107: F, t31057: F, t4210: F, t1095: F, t1980: F, t3120: F, t7476: F, t1967: F, t7658: F, t31058: F, t2087: F, t7780: F, t3806: F, t7741: F, t3055: F, t597: F) -> (F, F, F, F, F, F, F) {
    let t31202 = t31057 * t13287 * t2107 * t4210;
    let t31210 = t1980 * t7476 * t1095 * t3120;
    let t31212 = t1967 * t7658;
    let t31222 = t31057 * t13287 * t31058;
    let t31224 = t7780 * t2087;
    let t31226 = t7741 * t3806;
    let t31227 = 0.25724410870841842183e-2 * t31226;
    let t31228 = t3055 * t597;
    (t31202, t31210, t31212, t31222, t31224, t31227, t31228)
}
