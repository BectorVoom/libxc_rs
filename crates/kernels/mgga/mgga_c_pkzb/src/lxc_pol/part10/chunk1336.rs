//! MGGA_C_PKZB lxc pol — lxc_pol part 10 (v4rho4_2) CSE chunk 1336/1420 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part10_v4rho4_2_chunk1336<F: Float>(t5931: F, t9685: F, t751: F, t9633: F, t2036: F, t3669: F, t3650: F, t785: F, t25113: F, t133: F, t18301: F, t2019: F, t2140: F, t26378: F, t26643: F, t290: F, t2971: F, t2977: F, t2981: F, t3686: F, t6031: F, t759: F, t7832: F, t7844: F, t7864: F, t7868: F, t7874: F, t791: F, t793: F, t794: F, t9314: F, t9670: F, t9682: F) -> (F, F) {
    let t26646 = t5931 * t9685;
    let t26653 = t751 * t9633;
    let t26656 = t2036 * t3669;
    let t26659 = t785 * t3650;
    let t26667 = t2036 * t25113;
    let t26676 = -0.39512695097613069591e1 * t9670 * t18301 + 0.65854491829355115987e0 * t290 * t26643 + 0.65854491829355115987e0 * t26646 * t7868 + 0.65854491829355115987e0 * t6031 * t3686 - 0.13170898365871023197e1 * t9682 * t7864 + 0.13170898365871023197e1 * t26653 * t794 - 0.65854491829355115987e0 * t26656 * t2140 + 0.26341796731742046394e1 * t2019 * t26659 * t2971 + 0.65854491829355115987e0 * t791 * t26378 * t133 * t793 - 0.13170898365871023197e1 * t26667 * t2981 + 0.26341796731742046394e1 * t7874 * t2977 - 0.15805078039045227836e2 * t7844 * t7832 * t9314 * t759;
    (t26659, t26676)
}
