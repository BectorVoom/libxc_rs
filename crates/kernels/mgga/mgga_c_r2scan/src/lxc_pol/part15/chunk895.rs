//! MGGA_C_R2SCAN lxc pol — lxc_pol part 15 (v4rho3sigma_5) CSE chunk 895/1115 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part15_v4rho3sigma_5_chunk895<F: Float>(t11110: F, t333: F, t335: F, t337: F, t1083: F, t1085: F, t1087: F, t11092: F, t11106: F, t11108: F, t1310: F, t3390: F, t3394: F, t3398: F, t839: F, t339: F) -> (F, F, F, F, F) {
    let t11111 = t333 * t11110;
    let t11113 = t335 * t11110;
    let t11115 = t337 * t11110;
    let t11117 = -0.9214113627294e1 * t11092 - 0.18428227254588e2 * t3390 * t839 - 0.9214113627294e1 * t1083 * t1310 + 0.734774460522e2 * t3394 * t839 + 0.367387230261e2 * t1085 * t1310 - 0.7662840944824e2 * t3398 * t839 - 0.3831420472412e2 * t1087 * t1310 - 0.8704e0 * t11106 - 0.17408e1 * t11108 - 0.8704e0 * t11111 - 0.4607056813647e1 * t11113 + 0.122462410087e2 * t11115;
    let t11118 = t339 * t11110;
    (t11111, t11113, t11115, t11117, t11118)
}
