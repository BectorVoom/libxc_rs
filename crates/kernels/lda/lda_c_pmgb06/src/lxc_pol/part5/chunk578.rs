//! LDA_C_PMGB06 lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 578/1267 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part5_v4rho4_3_chunk578<F: Float>(t3703: F, t3738: F, t3741: F, t696: F, t968: F, t971: F, t1830: F, t2060: F, t3680: F, t3683: F, t3685: F, t3690: F, t3692: F, t3694: F) -> (F, F, F, F) {
    let t3742 = t3738 * t3703 * t3741;
    let t3744 = F::cast_from(1025.4018858216407_f64) * t696 * t3742;
    let t3748 = t971 * t968;
    let t3758 = -F::cast_from(3.4523333333333333_f64) * t3680 + F::cast_from(2.3015555555555554_f64) * t3683 - F::cast_from(2.6851481481481483_f64) * t3685 - F::cast_from(0.9393222222222222_f64) * t1830 + F::cast_from(0.073355_f64) * t3690 - F::cast_from(0.14671_f64) * t3692 - F::cast_from(0.17116166666666666_f64) * t3694 - F::cast_from(0.36793333333333333_f64) * t2060;
    (t3742, t3744, t3748, t3758)
}
