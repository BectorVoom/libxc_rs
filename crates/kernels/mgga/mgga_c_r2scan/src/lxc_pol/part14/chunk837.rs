//! MGGA_C_R2SCAN lxc pol — lxc_pol part 14 (v4rho3sigma_4) CSE chunk 837/1124 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part14_v4rho3sigma_4_chunk837<F: Float>(t1020: F, t1312: F, t1022: F, t1024: F, t1026: F, t1028: F, t1310: F, t2414: F, t2418: F, t2422: F, t839: F, t8438: F, t2410: F, t333: F, t335: F, t337: F) -> (F, F, F, F, F, F, F) {
    let t8440 = t1312 * t1020;
    let t8454 = 0.734774460522e2 * t1022 * t1312 - 0.11494261417236e3 * t1024 * t1312 + 0.6202613620464e2 * t1026 * t1312 - 0.1088826475632e2 * t1028 * t1312 - 0.64e0 * t8438 - 0.9214113627294e1 * t8440 - 0.18428227254588e2 * t2414 * t839 - 0.9214113627294e1 * t1022 * t1310 + 0.734774460522e2 * t2418 * t839 + 0.367387230261e2 * t1024 * t1310 - 0.7662840944824e2 * t2422 * t839 - 0.3831420472412e2 * t1026 * t1310;
    let t8463 = t1310 * t1020;
    let t8465 = t839 * t2410;
    let t8467 = t333 * t8438;
    let t8469 = t335 * t8438;
    let t8471 = t337 * t8438;
    (t8440, t8454, t8463, t8465, t8467, t8469, t8471)
}
