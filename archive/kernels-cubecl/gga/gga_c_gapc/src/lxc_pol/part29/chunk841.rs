//! GGA_C_GAPC lxc pol — lxc_pol part 29 (v4rho2sigma2_8) CSE chunk 841/1311 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part29_v4rho2sigma2_8_chunk841<F: Float>(t1936: F, t2520: F, t3345: F, t3424: F, t3427: F, t3431: F, t7675: F, t3437: F, t8915: F, t3440: F, t9731: F, t9734: F, t9742: F, t9745: F, t9748: F, t9751: F, t9754: F, t9758: F, t9761: F) -> (F, F) {
    let t9763 = t2520 * t1936;
    let t9764 = t9763 * t3345;
    let t9766 = t3424 * t3427;
    let t9768 = t7675 * t3431;
    let t9770 = t3437 * t8915;
    let t9771 = t9770 * t3440;
    let t9773 = F::cast_from(0.17376185052903442709e-3_f64) * t9731 - F::cast_from(0.19323635647535681159e-7_f64) * t9734 + F::cast_from(0.2152351096824363426e-6_f64) * t9742 + F::cast_from(0.86880925264517213544e-4_f64) * t9745 - F::cast_from(0.69504740211613770836e-4_f64) * t9748 - F::cast_from(0.69504740211613770836e-4_f64) * t9751 + F::cast_from(0.2085142206348413125e-3_f64) * t9754 + F::cast_from(0.50027140879067581468e-9_f64) * t9758 + F::cast_from(0.77294542590142724635e-6_f64) * t9761 - F::cast_from(0.25745714186718600948e-5_f64) * t9764 - F::cast_from(0.17376185052903442709e-3_f64) * t9766 + F::cast_from(0.14480154210752868924e-5_f64) * t9768 - F::cast_from(0.22509399720615334744e-7_f64) * t9771;
    (t9770, t9773)
}
