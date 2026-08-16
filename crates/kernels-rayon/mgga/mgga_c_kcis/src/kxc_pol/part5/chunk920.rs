//! MGGA_C_KCIS kxc pol — kxc_pol part 5 (v3rho3_2) CSE chunk 920/1419 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_kxc_pol_part5_v3rho3_2_chunk920(t198: f64, t237: f64, t2664: f64, t2675: f64, t2681: f64, t2690: f64, t2694: f64, t2698: f64, t2701: f64, t2702: f64, t5: f64, t56: f64, t742: f64, t850: f64, t852: f64, t858: f64, t8669: f64, t8678: f64, t8798: f64, t8809: f64, t8812: f64, t8815: f64, t8816: f64, t8819: f64, t8824: f64, t8826: f64, t8829: f64, t8832: f64, t8833: f64, t8836: f64, t8845: f64) -> f64 {
    let t8849 = 0.96494049533612093922e2_f64 * t2681 * t8798 * t850 + 0.56969282336565386482e-3_f64 * t5 * t742 * t56 + 0.16562449037037037036e-2_f64 * t5 * t742 * t198 + 0.51947267698127589897e2_f64 * t2701 * t8809 - 0.35089340384731224426e1_f64 * t2694 * t8812 - 0.1038945353962551798e3_f64 * t8815 * t8816 + 0.58482233974552040708e0_f64 * t858 * t8819 - t8678 - 0.19298809906722418785e3_f64 * t8824 * t8826 + t8669 + 6.0_f64 * t2681 * t8829 + 0.1025389702100779493e4_f64 * t8832 * t8833 + 0.35089340384731224426e1_f64 * t2701 * t8836 - 6.0_f64 * t2664 * t852 * t2675 - 0.16265371324172286321e-1_f64 * t237 * t2690 * t2698 - 0.48159446095139119799e0_f64 * t237 * t8845 * t2702;
    t8849
}
