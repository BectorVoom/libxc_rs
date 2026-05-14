//! GGA_C_ACGGAP lxc pol — lxc_pol part 5 (v4rho4_2) CSE chunk 856/1191 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part5_v4rho4_2_chunk856<F: Float>(t14220: F, t3393: F, t1530: F, t3430: F, t1037: F, t1165: F, t3451: F, t930: F, t879: F, t944: F, t3253: F, t3456: F, t3375: F, t3665: F, t3775: F, t3806: F) -> (F, F, F, F, F, F, F) {
    let t14221 = t14220 * t3393;
    let t14223 = t1530 * t3430;
    let t14228 = t3451 * t1165 * t1037 * t930;
    let t14230 = t944 * t879;
    let t14233 = t3456 * t1165 * t3253 * t14230;
    let t14239 = t3375 * t3665;
    let t14242 = 0.51448821741683684368e-2 * t3775 * t3806;
    (t14221, t14223, t14228, t14230, t14233, t14239, t14242)
}
