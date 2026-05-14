//! MGGA_C_KCIS kxc pol — kxc_pol part 4 (v3rho3_1) CSE chunk 1152/1239 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_kxc_pol_part4_v3rho3_1_chunk1152<F: Float>(t4134: F, t531: F, t1650: F, t4136: F, t4170: F, t4160: F, t1363: F, t5623: F, t1466: F, t5869: F, t1490: F, t1464: F, t12274: F, t2013: F, t3738: F, t5757: F, sigma2: F) -> (F, F, F, F, F, F) {
    let t16735 = t4134 * t531;
    let t16737 = t16735 * t1650 * t4136;
    let t16738 = t4170 * t16737;
    let t16739 = t4160 * t16738;
    let t16744 = t5623 * t1363;
    let t16751 = t5869 * t1466;
    let t16752 = t16751 * sigma2;
    let t16753 = t16752 * t1490;
    let t16754 = t1464 * t16753;
    let t16756 = t12274 * t2013;
    let t16758 = t3738 * t5757;
    (t16739, t16744, t16751, t16754, t16756, t16758)
}
