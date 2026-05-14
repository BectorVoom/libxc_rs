//! LDA_C_PMGB06 lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 558/1097 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part5_v4rho4_3_chunk558<F: Float>(t3758: F, t675: F, t682: F, t696: F, t1066: F, t643: F, t638: F, t1065: F, t653: F, t248: F, t1830: F, t2060: F, t3680: F, t3683: F, t3685: F, t3690: F, t3692: F, t3694: F) -> (F, F, F, F, F, F, F, F) {
    let t3760 = t675 * t3758 * t682;
    let t3762 = 0.5848223622634646 * t696 * t3760;
    let t3764 = 12.0 * t643 * t1066;
    let t3765 = t638 * t1066;
    let t3766 = 12.0 * t3765;
    let t3767 = t653 * t1065;
    let t3768 = t248 * t3767;
    let t3778 = -4.7063 * t3680 + 3.1375333333333333 * t3683 - 3.6604555555555556 * t3685 - 1.6068111111111112 * t1830 + 0.2805166666666667 * t3690 - 0.5610333333333334 * t3692 - 0.6545388888888889 * t3694 - 0.4630888888888889 * t2060;
    (t3760, t3762, t3764, t3765, t3766, t3767, t3768, t3778)
}
