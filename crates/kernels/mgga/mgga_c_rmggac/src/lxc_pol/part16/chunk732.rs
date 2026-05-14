//! MGGA_C_RMGGAC lxc pol — lxc_pol part 16 (v4rho3sigma_7) CSE chunk 732/1012 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part16_v4rho3sigma_7_chunk732<F: Float>(t674: F, t7715: F, t8687: F, t7243: F, t8576: F, t1973: F, t16156: F, t9138: F, t2310: F, t34881: F, t2313: F, t34855: F, t2305: F, t35326: F, t638: F, t7292: F, t8475: F) -> (F, F, F, F, F, F, F, F) {
    let t39281 = t8687 * t7715 * t674;
    let t39284 = t8576 * t7243;
    let t39285 = t39284 * t1973;
    let t39289 = t16156 * t9138;
    let t39295 = t34881 * t2310;
    let t39300 = t2313 * t34855 * t674;
    let t39308 = t35326 * t2305;
    let t39333 = t638 * t7292 * t8475;
    (t39281, t39284, t39285, t39289, t39295, t39300, t39308, t39333)
}
