//! MGGA_C_PKZB lxc pol — lxc_pol part 10 (v4rho4_2) CSE chunk 1174/1420 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part10_v4rho4_2_chunk1174<F: Float>(t2189: F, t2196: F, t18439: F, t2249: F, t2256: F, t6312: F, t858: F, t6121: F, t877: F, t18520: F, t369: F, t6230: F, t18509: F, t6287: F, t2277: F, t356: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t18796 = t2189 * t2196;
    let t18843 = 0.18467901234567901234e0 * t18439;
    let t18851 = t2249 * t2256;
    let t18854 = t858 * t6312;
    let t18863 = t877 * t6121;
    let t18866 = t369 * t18520;
    let t18875 = t877 * t6230;
    let t18878 = t369 * t18509;
    let t18882 = t858 * t6287;
    let t18885 = t2277 * t2277;
    let t18887 = t356 / t18885;
    (t18796, t18843, t18851, t18854, t18863, t18866, t18875, t18878, t18882, t18887)
}
