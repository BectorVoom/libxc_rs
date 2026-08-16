//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 2922/3259 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2922<F: Float>(t11199: F, t1646: F, t378: F, t1072: F, t994: F, t3046: F, t379: F, t11174: F, t11187: F, t11190: F, t11203: F, t11224: F, t11804: F, t12034: F, t12039: F, t16249: F, t16275: F, t16295: F, t16302: F, t16312: F, t16318: F, t16322: F, t16333: F, t16344: F, t16352: F, t1647: F, t1652: F, t16605: F, t1696: F, t19428: F, t3052: F, t3063: F, t3067: F, t3271: F, t33768: F, t42038: F, t42044: F, t42052: F, t4747: F, t4941: F) -> (F, F) {
    let t53014 = t1646 * t11199;
    let t53015 = t53014 * t378;
    let t53027 = t994 * t1072;
    let t53034 = t3046 * t379;
    let t53056 = F::cast_from(0.39512695097613069591e1_f64) * t16333 * t3271 - F::cast_from(0.39512695097613069591e1_f64) * t53015 * t11203 - F::cast_from(0.65854491829355115987e0_f64) * t42038 * t1696 + F::cast_from(0.65854491829355115987e0_f64) * t1647 * t12034 - F::cast_from(0.11853808529283920877e2_f64) * t3052 * t16322 + F::cast_from(0.79025390195226139182e1_f64) * t16312 * t19428 * t12039 - F::cast_from(0.79025390195226139182e1_f64) * t53027 * t16605 - F::cast_from(0.39512695097613069591e1_f64) * t3063 * t16249 + F::cast_from(0.19756347548806534796e1_f64) * t11190 * t4941 - F::cast_from(0.79025390195226139182e1_f64) * t53034 * t16605 - F::cast_from(0.19756347548806534796e1_f64) * t42044 * t1652 + F::cast_from(0.39512695097613069591e1_f64) * t3052 * t16318 - F::cast_from(0.65854491829355115987e0_f64) * t4747 * t11174 - F::cast_from(0.39512695097613069591e1_f64) * t16312 * t33768 * t11804 - F::cast_from(0.11853808529283920877e2_f64) * t42052 * t16275 + F::cast_from(0.19756347548806534796e1_f64) * t3063 * t16352 + F::cast_from(0.39512695097613069591e1_f64) * t11224 * t16295 - F::cast_from(0.39512695097613069591e1_f64) * t11187 * t16344 + F::cast_from(0.39512695097613069591e1_f64) * t16302 * t3067;
    (t53014, t53056)
}
