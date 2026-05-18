//! MGGA_C_R2SCAN lxc pol — lxc_pol part 14 (v4rho3sigma_4) CSE chunk 910/1276 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part14_v4rho3sigma_4_chunk910<F: Float>(t1020: F, t1312: F, t1022: F, t1024: F, t1026: F, t1028: F, t1310: F, t2414: F, t2418: F, t2422: F, t839: F, t8438: F) -> (F, F) {
    let t8440 = t1312 * t1020;
    let t8454 = F::new(0.734774460522e2) * t1022 * t1312 - F::new(0.11494261417236e3) * t1024 * t1312 + F::new(0.6202613620464e2) * t1026 * t1312 - F::new(0.1088826475632e2) * t1028 * t1312 - F::new(0.64e0) * t8438 - F::new(0.9214113627294e1) * t8440 - F::new(0.18428227254588e2) * t2414 * t839 - F::new(0.9214113627294e1) * t1022 * t1310 + F::new(0.734774460522e2) * t2418 * t839 + F::new(0.367387230261e2) * t1024 * t1310 - F::new(0.7662840944824e2) * t2422 * t839 - F::new(0.3831420472412e2) * t1026 * t1310;
    (t8440, t8454)
}
