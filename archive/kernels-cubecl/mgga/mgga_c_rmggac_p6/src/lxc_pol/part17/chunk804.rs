//! MGGA_C_RMGGAC lxc pol — lxc_pol part 17 (v4rho3sigma_8) CSE chunk 804/1111 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part17_v4rho3sigma_8_chunk804<F: Float>(t674: F, t7715: F, t8687: F, t7243: F, t8576: F, t1973: F, t16156: F, t9138: F, t2310: F, t34881: F, t2313: F, t34855: F) -> (F, F, F, F, F, F) {
    let t39281 = t8687 * t7715 * t674;
    let t39284 = t8576 * t7243;
    let t39285 = t39284 * t1973;
    let t39286 = F::cast_from(0.19863479950205658386e-4_f64) * t39285;
    let t39289 = t16156 * t9138;
    let t39290 = F::cast_from(0.39726959900411316772e-4_f64) * t39289;
    let t39295 = t34881 * t2310;
    let t39296 = F::cast_from(0.19863479950205658386e-4_f64) * t39295;
    let t39300 = t2313 * t34855 * t674;
    (t39281, t39284, t39286, t39290, t39296, t39300)
}
