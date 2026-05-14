//! MGGA_C_KCIS lxc pol — lxc_pol part 5 (v3rho3_2) CSE chunk 856/1260 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part5_v3rho3_2_chunk856<F: Float>(t2683: F, t8825: F, t851: F, t47: F, t8655: F, t8656: F, t8659: F, t680: F, t2372: F, t88: F, t198: F, t237: F, t2664: F, t2675: F, t2681: F, t2690: F, t2694: F, t2698: F, t2701: F, t2702: F, t5: F, t56: F, t742: F, t850: F, t852: F, t858: F, t8669: F, t8678: F, t8798: F, t8809: F, t8812: F, t8815: F, t8816: F, t8819: F, t8824: F) -> (F,) {
    let t8826 = t8825 * t2683;
    let t8829 = t8825 * t851;
    let t8832 = t47 * t8655;
    let t8833 = t8656 * t8659;
    let t8836 = t8656 * t680;
    let t8845 = t88 * t2372;
    let t8849 = 0.96494049533612093922e2 * t2681 * t8798 * t850 + 0.56969282336565386482e-3 * t5 * t742 * t56 + 0.16562449037037037036e-2 * t5 * t742 * t198 + 0.51947267698127589897e2 * t2701 * t8809 - 0.35089340384731224426e1 * t2694 * t8812 - 0.1038945353962551798e3 * t8815 * t8816 + 0.58482233974552040708e0 * t858 * t8819 - t8678 - 0.19298809906722418785e3 * t8824 * t8826 + t8669 + 6.0 * t2681 * t8829 + 0.1025389702100779493e4 * t8832 * t8833 + 0.35089340384731224426e1 * t2701 * t8836 - 6.0 * t2664 * t852 * t2675 - 0.16265371324172286321e-1 * t237 * t2690 * t2698 - 0.48159446095139119799e0 * t237 * t8845 * t2702;
    (t8849,)
}
