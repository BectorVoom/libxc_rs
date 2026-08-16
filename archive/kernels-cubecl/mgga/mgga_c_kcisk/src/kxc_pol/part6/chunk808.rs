//! MGGA_C_KCISK kxc pol — kxc_pol part 6 (v3rho3_3) CSE chunk 808/1086 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_kxc_pol_part6_v3rho3_3_chunk808<F: Float>(t4811: F, t8678: F, t5074: F, t8951: F, t1333: F, t8862: F, t1907: F, t8964: F, t1871: F, t9014: F, t4265: F, t8999: F, sigma2: F) -> (F, F, F, F, F, F) {
    let t23976 = t4811 * t8678;
    let t23978 = t5074 * t8951;
    let t24073 = t1333 * t8862;
    let t24081 = t8964 * t1907;
    let t24202 = t9014 * t1871;
    let t24203 = t24202 * sigma2;
    let t24299 = t4265 * t8999;
    (t23976, t23978, t24073, t24081, t24203, t24299)
}
