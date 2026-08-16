//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 574/1335 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk574(t13: f64, t3127: f64, t30: f64, t906: f64, t3122: f64, t27: f64, t902: f64, t907: f64, t1953: f64, t2061: f64, t2717: f64, t2720: f64, t2723: f64, t2728: f64, t2730: f64, t2732: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t3128 = t13 * t3127;
    let t3130 = 1.0_f64 / t906 / t30;
    let t3131 = t3122 * t3130;
    let t3132 = t3128 * t3131;
    let t3133 = 517.2501470570617_f64 * t3132;
    let t3135 = 1.0_f64 / t902 / t27;
    let t3136 = t13 * t3135;
    let t3137 = t3122 * t907;
    let t3138 = t3136 * t3137;
    let t3139 = 96.49094593290663_f64 * t3138;
    let t3148 = -2.5319_f64 * t2717 + 1.6879333333333333_f64 * t2720 - 1.9692555555555555_f64 * t2723 - 0.9301185185185186_f64 * t1953 + 0.13651666666666668_f64 * t2728 - 0.27303333333333335_f64 * t2730 - 0.31853888888888887_f64 * t2732 - 0.36514074074074077_f64 * t2061;
    (t3128, t3130, t3131, t3132, t3133, t3135, t3136, t3137, t3138, t3139, t3148)
}
