//! GGA_C_GAPLOC lxc pol — lxc_pol part 43 (v4rhosigma3_8) CSE chunk 855/923 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part43_v4rhosigma3_8_chunk855<F: Float>(t47918: F, t536: F, t40192: F, t40196: F, t12054: F, t9333: F, t12065: F, t2437: F, t2441: F, t38759: F, t895: F, t10348: F, t13779: F, t1407: F, t9285: F, t447: F, t46849: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
    let t47920 = 0.35750489951850426669e0 * t536 * t47918;
    let t47925 = 0.38342925953920749677e0 * t40192;
    let t47926 = 0.85206502119823888171e-1 * t40196;
    let t47927 = t12054 * t9333;
    let t47934 = t2437 * t12065;
    let t47937 = t2441 * t12065;
    let t47939 = t895 * t38759;
    let t47941 = t12054 * t10348;
    let t47949 = t1407 * t13779;
    let t47951 = t9285 * t12065;
    let t47953 = t46849 * t447;
    (t47920, t47925, t47926, t47927, t47934, t47937, t47939, t47941, t47949, t47951, t47953)
}
