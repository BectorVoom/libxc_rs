//! LDA_C_PMGB06 lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 579/1267 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part5_v4rho4_3_chunk579(t3758: f64, t675: f64, t682: f64, t696: f64, t1066: f64, t643: f64, t638: f64, t1065: f64, t653: f64, t248: f64, t1830: f64, t2060: f64, t3680: f64, t3683: f64, t3685: f64, t3690: f64, t3692: f64, t3694: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t3760 = t675 * t3758 * t682;
    let t3762 = 0.5848223622634646_f64 * t696 * t3760;
    let t3764 = 12.0_f64 * t643 * t1066;
    let t3765 = t638 * t1066;
    let t3766 = 12.0_f64 * t3765;
    let t3767 = t653 * t1065;
    let t3768 = t248 * t3767;
    let t3778 = -4.7063_f64 * t3680 + 3.1375333333333333_f64 * t3683 - 3.6604555555555556_f64 * t3685 - 1.6068111111111112_f64 * t1830 + 0.2805166666666667_f64 * t3690 - 0.5610333333333334_f64 * t3692 - 0.6545388888888889_f64 * t3694 - 0.4630888888888889_f64 * t2060;
    (t3760, t3762, t3764, t3765, t3766, t3767, t3768, t3778)
}
