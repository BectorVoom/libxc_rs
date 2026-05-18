//! GGA_C_ACGGAP lxc pol — lxc_pol part 12 (v4rho3sigma_4) CSE chunk 980/1250 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part12_v4rho3sigma_4_chunk980<F: Float>(t8111: F, t872: F, t2217: F, t323: F, t851: F, t32010: F, t7963: F, t8306: F, t16548: F, t7942: F, t2176: F, t3909: F) -> (F, F, F, F, F) {
    let t33065 = t8111 * t872;
    let t33080 = t851 * t2217 * t323;
    let t33085 = t7963 * t8306 * t32010;
    let t33088 = t7942 * t8306 * t16548;
    let t33090 = t2176 * t3909;
    (t33065, t33080, t33085, t33088, t33090)
}
