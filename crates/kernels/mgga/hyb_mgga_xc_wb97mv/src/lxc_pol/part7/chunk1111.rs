//! HYB_MGGA_XC_WB97MV lxc pol — lxc_pol part 7 (v4rho4_2) CSE chunk 1111/1375 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::erf::{erf_approx};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn hyb_mgga_xc_wb97mv_lxc_pol_part7_v4rho4_2_chunk1111<F: Float>(t1114: F, t1537: F, t3736: F, t4077: F, t646: F, t3686: F, t1291: F, t3678: F, t3677: F, t518: F, t5395: F, t2952: F, t3740: F, t10080: F, t3791: F, t10039: F, t10054: F, t11822: F, t11827: F, t11834: F, t11837: F, t11840: F, t11843: F, t11846: F, t11849: F, t11854: F, t2817: F, t2823: F, t2828: F, t2832: F, t3680: F, t3771: F, t3785: F, t7818: F, sigma0: F) -> (F, F, F, F, F, F, F, F) {
    let t11857 = t1537 * t1114;
    let t11858 = t3736 * t11857;
    let t11861 = t4077 * sigma0;
    let t11862 = t11861 * t646;
    let t11863 = t3686 * t11862;
    let t11866 = t3678 * t1291;
    let t11867 = t3677 * t11866;
    let t11876 = t5395 * t518;
    let t11879 = t2952 * t3740;
    let t11880 = t10080 * t3791;
    let t11883 = -0.48e-4 * t2823 * t11827 + 0.144e-3 * t2828 * t11822 - 0.144e-3 * t2832 * t11827 + 1400.0 / 3.0 * t11834 * t3785 + 200.0 / 9.0 * t11837 * t3785 + 400.0 / 9.0 * t10039 * t11840 - 400.0 / 9.0 * t11843 * t3785 + 200.0 / 3.0 * t11846 * t3785 - 1000.0 / 3.0 * t11849 * t3785 + 400.0 * t10054 * t11840 - 400.0 * t11854 * t3785 + 0.2016e-2 * t7818 * t11858 - 0.1408e-5 * t3771 * t11863 + 0.88888888888888888889e-1 * t2817 * t11867 + 0.88888888888888888889e-1 * t2823 * t11867 + 0.26666666666666666667e0 * t2828 * t11867 + 0.26666666666666666667e0 * t2832 * t11867 - 0.66666666666666666667e-1 * t11876 * t3680 - 0.48e0 * t11879 * t11880;
    (t11857, t11858, t11861, t11863, t11867, t11876, t11879, t11883)
}
