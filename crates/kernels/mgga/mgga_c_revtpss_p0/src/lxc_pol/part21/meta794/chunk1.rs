//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 2874/3259 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2874<F: Float>(t11379: F, t2924: F, t4635: F, t11300: F, t1609: F, t41499: F, t41502: F, t11528: F, t15383: F, t15386: F, t41883: F, t11294: F, t15393: F) -> (F, F, F, F, F) {
    let t52170 = F::cast_from(0.16081979498692535067e2_f64) * t2924 * t4635 * t11379;
    let t52174 = F::cast_from(0.24955700379505800916e5_f64) * t41499 * t1609 * t41502 * t11300;
    let t52176 = F::new(6.0) * t11528 * t15383;
    let t52178 = F::cast_from(0.28947563097646563121e3_f64) * t41883 * t15386;
    let t52180 = F::cast_from(0.48245938496077605201e2_f64) * t11294 * t15393;
    (t52170, t52174, t52176, t52178, t52180)
}
