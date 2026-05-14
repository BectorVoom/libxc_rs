//! MGGA_C_KCISK kxc pol — kxc_pol part 3 (v3rho3_0) CSE chunk 597/938 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_kxc_pol_part3_v3rho3_0_chunk597<F: Float>(t673: F, t707: F, t1744: F, t4957: F, t4971: F, t638: F, t5005: F, t9: F, t662: F, t5014: F, t1310: F, t657: F, t718: F, t733: F, t1755: F, t41: F) -> (F, F, F, F, F, F, F, F, F) {
    let t7055 = t673 * t707;
    let t7181 = t4957 * t1744;
    let t7196 = t638 * t4971;
    let t7233 = t9 * t5005;
    let t7234 = t7233 * t662;
    let t7242 = t5014 * t662;
    let t7261 = t1310 * t657;
    let t7302 = t733 * t718;
    let t7303 = t41 * t1755;
    (t7055, t7181, t7196, t7233, t7234, t7242, t7261, t7302, t7303)
}
