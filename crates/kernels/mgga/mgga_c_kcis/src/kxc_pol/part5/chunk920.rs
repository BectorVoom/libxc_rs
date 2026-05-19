//! MGGA_C_KCIS kxc pol — kxc_pol part 5 (v3rho3_2) CSE chunk 920/1419 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_kxc_pol_part5_v3rho3_2_chunk920<F: Float>(t198: F, t237: F, t2664: F, t2675: F, t2681: F, t2690: F, t2694: F, t2698: F, t2701: F, t2702: F, t5: F, t56: F, t742: F, t850: F, t852: F, t858: F, t8669: F, t8678: F, t8798: F, t8809: F, t8812: F, t8815: F, t8816: F, t8819: F, t8824: F, t8826: F, t8829: F, t8832: F, t8833: F, t8836: F, t8845: F) -> F {
    let t8849 = F::cast_from(0.96494049533612093922e2_f64) * t2681 * t8798 * t850 + F::cast_from(0.56969282336565386482e-3_f64) * t5 * t742 * t56 + F::cast_from(0.16562449037037037036e-2_f64) * t5 * t742 * t198 + F::cast_from(0.51947267698127589897e2_f64) * t2701 * t8809 - F::cast_from(0.35089340384731224426e1_f64) * t2694 * t8812 - F::cast_from(0.1038945353962551798e3_f64) * t8815 * t8816 + F::cast_from(0.58482233974552040708e0_f64) * t858 * t8819 - t8678 - F::cast_from(0.19298809906722418785e3_f64) * t8824 * t8826 + t8669 + F::new(6.0) * t2681 * t8829 + F::cast_from(0.1025389702100779493e4_f64) * t8832 * t8833 + F::cast_from(0.35089340384731224426e1_f64) * t2701 * t8836 - F::new(6.0) * t2664 * t852 * t2675 - F::cast_from(0.16265371324172286321e-1_f64) * t237 * t2690 * t2698 - F::cast_from(0.48159446095139119799e0_f64) * t237 * t8845 * t2702;
    t8849
}
