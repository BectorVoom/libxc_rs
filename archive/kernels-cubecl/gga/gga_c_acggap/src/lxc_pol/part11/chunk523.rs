//! GGA_C_ACGGAP lxc pol — lxc_pol part 11 (v4rho3sigma_3) CSE chunk 523/1213 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part11_v4rho3sigma_3_chunk523<F: Float>(t3143: F, t347: F, t1049: F, t1065: F, t227: F, t8: F, t130: F, t134: F, t14: F, t2: F, t41: F, t135: F) -> (F, F, F, F, F, F) {
    let t3144 = t3143 * t347;
    let t3146 = t1049 * t1065;
    let t3151 = F::cast_from(1.0_f64) / t8 / t227;
    let t3152 = t130 * t3151;
    let t3153 = t3152 * t134;
    let t3157 = F::cast_from(1.0_f64) / t14 / t2 / t41 / F::cast_from(48.0_f64);
    let t3159 = t135 * t3157 * t2;
    (t3144, t3146, t3151, t3153, t3157, t3159)
}
