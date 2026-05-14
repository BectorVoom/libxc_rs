//! GGA_C_FT97 lxc pol — lxc_pol part 15 (v4rho4_4) CSE chunk 712/1067 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part15_v4rho4_4_chunk712<F: Float>(t21399: F, t676: F, t27: F, t89: F, t1091: F, t4934: F, t9770: F, t446: F, t1131: F, t4969: F, t2354: F, t20489: F, t669: F, t666: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
    let t21400 = t676 * t21399;
    let t21402 = t89 * t27 * t21400;
    let t21404 = t1091 * t4934;
    let t21405 = t9770 * t21404;
    let t21406 = t446 * t21405;
    let t21408 = t4969 * t1131;
    let t21409 = t2354 * t21408;
    let t21410 = t446 * t21409;
    let t21412 = t669 * t20489;
    let t21414 = t89 * t666 * t21412;
    let t21416 = t4934 * t1131;
    (t21400, t21402, t21404, t21405, t21406, t21408, t21409, t21410, t21412, t21414, t21416)
}
