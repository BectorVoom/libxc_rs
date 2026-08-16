//! GGA_C_GAPC lxc pol — lxc_pol part 33 (v4rho2sigma2_12) CSE chunk 838/1306 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gapc_lxc_pol_part33_v4rho2sigma2_12_chunk838(t1936: f64, t2520: f64, t3345: f64, t3424: f64, t3427: f64, t3431: f64, t7675: f64, t3437: f64, t8915: f64, t3440: f64, t9731: f64, t9734: f64, t9742: f64, t9745: f64, t9748: f64, t9751: f64, t9754: f64, t9758: f64, t9761: f64) -> (f64, f64) {
    let t9763 = t2520 * t1936;
    let t9764 = t9763 * t3345;
    let t9766 = t3424 * t3427;
    let t9768 = t7675 * t3431;
    let t9770 = t3437 * t8915;
    let t9771 = t9770 * t3440;
    let t9773 = 0.17376185052903442709e-3_f64 * t9731 - 0.19323635647535681159e-7_f64 * t9734 + 0.2152351096824363426e-6_f64 * t9742 + 0.86880925264517213544e-4_f64 * t9745 - 0.69504740211613770836e-4_f64 * t9748 - 0.69504740211613770836e-4_f64 * t9751 + 0.2085142206348413125e-3_f64 * t9754 + 0.50027140879067581468e-9_f64 * t9758 + 0.77294542590142724635e-6_f64 * t9761 - 0.25745714186718600948e-5_f64 * t9764 - 0.17376185052903442709e-3_f64 * t9766 + 0.14480154210752868924e-5_f64 * t9768 - 0.22509399720615334744e-7_f64 * t9771;
    (t9770, t9773)
}
