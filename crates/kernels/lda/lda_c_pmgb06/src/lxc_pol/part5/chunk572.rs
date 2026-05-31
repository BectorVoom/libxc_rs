//! LDA_C_PMGB06 lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 572/1267 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part5_v4rho4_3_chunk572<F: Float>(t740: F, t934: F, t940: F, t2781: F, t623: F, t36: F, t28: F, t247: F, t950: F, t628: F, t1830: F, t2060: F, t3680: F) -> (F, F, F, F, F, F, F) {
    let t3682 = t934 * t740;
    let t3683 = t940 * t3682;
    let t3685 = t623 * t2781;
    let t3688 = F::cast_from(1.0_f64)/pow_3_2::<F>(t36);
    let t3689 = t3688 * t28;
    let t3690 = t3689 * t247;
    let t3692 = t950 * t3682;
    let t3694 = t628 * t2781;
    let t3697 = -F::cast_from(2.5319_f64) * t3680 + F::cast_from(1.6879333333333333_f64) * t3683 - F::cast_from(1.9692555555555555_f64) * t3685 - F::cast_from(0.9301185185185186_f64) * t1830 + F::cast_from(0.13651666666666668_f64) * t3690 - F::cast_from(0.27303333333333335_f64) * t3692 - F::cast_from(0.31853888888888887_f64) * t3694 - F::cast_from(0.36514074074074077_f64) * t2060;
    (t3683, t3685, t3689, t3690, t3692, t3694, t3697)
}
