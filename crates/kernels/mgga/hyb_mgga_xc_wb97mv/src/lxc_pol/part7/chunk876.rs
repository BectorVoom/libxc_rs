//! HYB_MGGA_XC_WB97MV lxc pol — lxc_pol part 7 (v4rho4_2) CSE chunk 876/1375 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::erf::{erf_approx};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn hyb_mgga_xc_wb97mv_lxc_pol_part7_v4rho4_2_chunk876<F: Float>(t1035: F, t2659: F, t2665: F, t2667: F, t1064: F, t1071: F, t1086: F, t222: F, t2724: F, t2729: F, t2732: F, t2742: F, t2747: F, t2750: F, t2760: F, t2765: F, t2768: F, t2772: F, t566: F, t7634: F, t7639: F, t7640: F, t7645: F, t7649: F, t7653: F, t7657: F, t7658: F, t7662: F, t7669: F, t7673: F, t7684: F, t7690: F) -> (F, F) {
    let t7694 = 0.48245938496077605201e2 * t2665 * t2659 * t2667 * t1035;
    let t7695 = 1.0 * t1064 * t7634 - t7639 + 6.0 * t2747 * t7640 + t7645 + t7649 - t7653 - t7657 - 0.48159733137676571078e0 * t222 * t7658 * t2772 + 0.21687162600603479684e-1 * t222 * t7662 * t1086 - 0.16265371950452609763e-1 * t222 * t2760 * t2768 - 0.16522625736956710527e1 * t222 * t7669 * t2750 + 0.68493333333333333332e-1 * t222 * t7673 * t1071 - 0.51369999999999999999e-1 * t222 * t2724 * t2742 + 0.10274e0 * t222 * t566 * t2729 * t2732 + 0.32530743900905219526e-1 * t222 * t7684 * t2765 - t7690 - t7694;
    (t7694, t7695)
}
