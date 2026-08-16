//! MGGA_C_R2SCAN lxc pol — lxc_pol part 17 (v4rho3sigma_7) CSE chunk 1167/1293 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part17_v4rho3sigma_7_chunk1167<F: Float>(t10760: F, t29726: F, t6535: F, t11720: F, t26282: F, t1058: F, t1060: F, t2207: F, t8629: F, t11780: F, t2201: F, t3602: F) -> (F, F, F, F) {
    let t43203 = t6535 * t10760 * t29726;
    let t43205 = t26282 * t11720;
    let t43209 = t2207 * t1058 * t1060 * t8629;
    let t43215 = t2201 * t11780 * t3602;
    (t43203, t43205, t43209, t43215)
}
