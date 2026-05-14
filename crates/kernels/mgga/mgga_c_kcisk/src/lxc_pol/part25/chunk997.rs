//! MGGA_C_KCISK lxc pol — lxc_pol part 25 (v4rho3sigma_5) CSE chunk 997/1395 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part25_v4rho3sigma_5_chunk997<F: Float>(t4954: F, t7175: F, t7181: F, t45: F, t7147: F, t10913: F, t2430: F, t4929: F, t4957: F, t10893: F, t7156: F, t10918: F, t1726: F, t1735: F, t1747: F, t17562: F, t17567: F, t17571: F, t17575: F, t17626: F, t17645: F, t2432: F, t4904: F, t4912: F, t4924: F, t4950: F, t4958: F, t621: F, t7091: F, t7151: F, t7158: F, t7177: F, t7182: F) -> (F,) {
    let t17648 = t4954 * t7175;
    let t17649 = t17648 * t7181;
    let t17656 = t45 * t7147;
    let t17667 = t10913 * t2430;
    let t17668 = t4957 * t4929;
    let t17669 = t17667 * t17668;
    let t17672 = t7156 * t10893;
    let t17675 = 2.0 * t17562 * t1726 + 1.0 * t7091 * t4904 + 0.16081824322151104822e2 * t17567 * t4912 - 0.35089340384731224426e1 * t1735 * t17571 - 0.17315755899375863299e2 * t1735 * t17575 - 0.58482233974552040708e0 * t1735 * t17626 - 0.62182e-1 * t17645 * t621 - 0.34631511798751726598e2 * t1735 * t17649 - 0.34631511798751726598e2 * t4924 * t7182 - 0.11696446794910408142e1 * t4924 * t7177 - 0.11696446794910408142e1 * t17656 * t1747 - 0.58482233974552040708e0 * t7151 * t4950 - 0.17315755899375863299e2 * t7151 * t4958 - 0.58482233974552040708e0 * t10918 * t2432 + 0.23392893589820816284e1 * t4924 * t7158 + 0.1038945353962551798e3 * t1735 * t17669 + 0.11696446794910408142e1 * t1735 * t17672;
    (t17675,)
}
