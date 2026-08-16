//! MGGA_C_RMGGAC lxc pol — lxc_pol part 34 (v4rho2sigma2_7) CSE chunk 972/1097 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part34_v4rho2sigma2_7_chunk972<F: Float>(t74674: F, t16503: F, t2211: F, t34976: F, t8435: F, t15450: F, t34761: F, t1502: F, t699: F, t74684: F, t74687: F, t74690: F) -> (F, F, F, F, F, F, F) {
    let t77143 = F::cast_from(0.1702583995731913576e-4_f64) * t74674;
    let t77147 = t16503 * t34976 * t2211 * t8435;
    let t77148 = F::cast_from(0.85129199786595678796e-5_f64) * t77147;
    let t77149 = t34761 * t15450;
    let t77150 = F::cast_from(0.42564599893297839398e-5_f64) * t77149;
    let t77153 = t16503 * t34976 * t699 * t1502;
    let t77154 = F::cast_from(0.42564599893297839398e-5_f64) * t77153;
    let t77155 = F::cast_from(0.2553875993597870364e-4_f64) * t74684;
    let t77156 = F::cast_from(0.2553875993597870364e-4_f64) * t74687;
    let t77157 = F::cast_from(0.3830813990396805546e-4_f64) * t74690;
    (t77143, t77148, t77150, t77154, t77155, t77156, t77157)
}
