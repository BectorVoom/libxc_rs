//! MGGA_C_R2SCAN lxc pol — lxc_pol part 17 (v4rho3sigma_7) CSE chunk 1029/1120 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part17_v4rho3sigma_7_chunk1029<F: Float>(t11780: F, t2207: F, t3606: F, t10760: F, t22820: F, t29279: F, t29726: F, t6535: F, t11720: F, t26282: F, t1058: F, t1060: F, t8629: F, t2201: F, t3602: F, t10856: F, t9423: F) -> (F, F, F, F, F, F, F) {
    let t43195 = t2207 * t11780 * t3606;
    let t43200 = t22820 * t10760 * t29279;
    let t43203 = t6535 * t10760 * t29726;
    let t43205 = t26282 * t11720;
    let t43209 = t2207 * t1058 * t1060 * t8629;
    let t43215 = t2201 * t11780 * t3602;
    let t43217 = t10856 * t9423;
    (t43195, t43200, t43203, t43205, t43209, t43215, t43217)
}
