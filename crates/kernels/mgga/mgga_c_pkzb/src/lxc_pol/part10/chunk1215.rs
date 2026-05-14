//! MGGA_C_PKZB lxc pol — lxc_pol part 10 (v4rho4_2) CSE chunk 1215/1420 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part10_v4rho4_2_chunk1215<F: Float>(t2278: F, t3080: F, t1189: F, t6287: F, t2196: F, t3030: F, t1171: F, t6141: F, t2256: F, t6312: F, t3069: F, t6201: F, t2320: F, t8098: F, t1235: F, t5722: F) -> (F, F, F, F, F, F, F, F, F) {
    let t22762 = t3080 * t2278;
    let t22767 = t1189 * t6287;
    let t22820 = t3030 * t2196;
    let t22823 = t1171 * t6141;
    let t22826 = t3080 * t2256;
    let t22829 = t1189 * t6312;
    let t22841 = t3069 * t6201;
    let t22868 = t8098 * t2320;
    let t22919 = t1235 * t5722;
    (t22762, t22767, t22820, t22823, t22826, t22829, t22841, t22868, t22919)
}
