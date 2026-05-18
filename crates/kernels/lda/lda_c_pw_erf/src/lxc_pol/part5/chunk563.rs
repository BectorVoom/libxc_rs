//! LDA_C_PW_ERF lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 563/1365 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part5_v4rho4_3_chunk563<F: Float>(t13: F, t3127: F, t30: F, t906: F, t3122: F, t27: F, t902: F, t907: F, t1953: F, t2061: F, t2717: F, t2720: F, t2723: F, t2728: F, t2730: F, t2732: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
    let t3128 = t13 * t3127;
    let t3130 = F::new(1.0) / t906 / t30;
    let t3131 = t3122 * t3130;
    let t3132 = t3128 * t3131;
    let t3133 = F::new(517.2501470570617) * t3132;
    let t3135 = F::new(1.0) / t902 / t27;
    let t3136 = t13 * t3135;
    let t3137 = t3122 * t907;
    let t3138 = t3136 * t3137;
    let t3139 = F::new(96.49094593290663) * t3138;
    let t3148 = -F::new(2.5319) * t2717 + F::new(1.6879333333333333) * t2720 - F::new(1.9692555555555555) * t2723 - F::new(0.9301185185185186) * t1953 + F::new(0.13651666666666668) * t2728 - F::new(0.27303333333333335) * t2730 - F::new(0.31853888888888887) * t2732 - F::new(0.36514074074074077) * t2061;
    (t3128, t3130, t3131, t3132, t3133, t3135, t3136, t3137, t3138, t3139, t3148)
}
