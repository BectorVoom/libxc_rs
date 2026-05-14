//! MGGA_C_PKZB lxc pol — lxc_pol part 9 (v4rho4_1) CSE chunk 1028/1213 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part9_v4rho4_1_chunk1028<F: Float>(t2189: F, t2196: F, t6352: F, t862: F, t18439: F, t2249: F, t2256: F, t6312: F, t858: F, t6131: F, t881: F, t6121: F, t877: F, t18520: F, t369: F, t6230: F) -> (F, F, F, F, F, F, F, F, F) {
    let t18796 = t2189 * t2196;
    let t18799 = t6352 * t862;
    let t18843 = 0.18467901234567901234e0 * t18439;
    let t18851 = t2249 * t2256;
    let t18854 = t858 * t6312;
    let t18860 = t6131 * t881;
    let t18863 = t877 * t6121;
    let t18866 = t369 * t18520;
    let t18875 = t877 * t6230;
    (t18796, t18799, t18843, t18851, t18854, t18860, t18863, t18866, t18875)
}
