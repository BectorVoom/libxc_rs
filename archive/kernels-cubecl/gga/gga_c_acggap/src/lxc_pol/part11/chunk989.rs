//! GGA_C_ACGGAP lxc pol — lxc_pol part 11 (v4rho3sigma_3) CSE chunk 989/1213 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part11_v4rho3sigma_3_chunk989<F: Float>(t2331: F, t943: F, t7908: F, t8998: F, t33489: F, t7963: F, t7965: F, t4210: F, t7942: F, t315: F, t5386: F, t610: F) -> (F, F, F, F, F) {
    let t33597 = t2331 * t943;
    let t33606 = F::cast_from(0.34694512752820797848e1_f64) * t8998 * t7908;
    let t33621 = F::cast_from(0.17347256376410398924e1_f64) * t7963 * t33489 * t7965;
    let t33624 = F::cast_from(0.17347256376410398924e1_f64) * t7942 * t33489 * t4210;
    let t33627 = F::cast_from(0.26341796731742046394e1_f64) * t315 * t610 * t5386;
    (t33597, t33606, t33621, t33624, t33627)
}
