//! MGGA_C_KCISK lxc pol — lxc_pol part 3 (v3rho3_0) CSE chunk 665/938 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part3_v3rho3_0_chunk665<F: Float>(t1724: F, t4903: F, t4911: F, t10903: F, t1746: F, t4954: F, t10978: F, t10983: F, t10984: F, t11067: F, t11102: F, t11116: F, t11119: F, t11122: F, t11125: F, t1706: F, t1735: F, t4853: F, t4858: F, t4860: F, t4904: F, t4909: F, t4912: F, t621: F) -> (F,) {
    let t11129 = t4903 * t4911 * t1724;
    let t11133 = t4954 * t10903 * t1746;
    let t11136 = 3.0 * t4853 * t4904 + 0.48245472966453314466e2 * t10978 * t4912 - 0.96490945932906628932e2 * t10983 * t10984 + 1.0 * t1706 * t11067 - 0.58482233974552040708e0 * t1735 * t11102 - 0.62182e-1 * t11116 * t621 - 6.0 * t11119 * t4860 + 6.0 * t4909 * t11122 - 6.0 * t4858 * t11125 + 0.48245472966453314466e2 * t4909 * t11129 - 0.35089340384731224426e1 * t1735 * t11133;
    (t11136,)
}
