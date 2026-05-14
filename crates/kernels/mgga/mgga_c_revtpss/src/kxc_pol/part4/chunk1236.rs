//! MGGA_C_REVTPSS kxc pol — kxc_pol part 4 (v3rho3_1) CSE chunk 1236/1274 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_kxc_pol_part4_v3rho3_1_chunk1236<F: Float>(t140: F, t3698: F, t5047: F, t1222: F, t1012: F, t13026: F, t16715: F, t16720: F, t5312: F, t1774: F, t3601: F, t3611: F, t3720: F, t12809: F, t12882: F, t12887: F, t12893: F, t12895: F, t12900: F, t12902: F, t12905: F) -> (F, F) {
    let t17471 = t140 * t3698;
    let t17472 = t17471 * t5047;
    let t17474 = t1222 * t17472 / 324.0;
    let t17475 = t1012 * t13026;
    let t17476 = t17475 * t16715;
    let t17479 = t5312 * t16720;
    let t17482 = t1774 * t3601;
    let t17483 = t17482 * t3611;
    let t17484 = t3720 * t17483;
    let t17493 = t17474 - 7.0 / 648.0 * t1222 * t17476 + t1222 * t17479 / 36.0 + 0.21437009059034868486e-3 * t12809 * t17484 + 0.63517063878621832551e-4 * t12882 + 0.15879265969655458138e-3 * t12887 - 0.95275595817932748826e-4 * t12893 + 0.14291339372689912324e-3 * t12895 + t12900 + 0.28582678745379824648e-3 * t12902 - 0.95275595817932748826e-4 * t12905;
    (t17482, t17493)
}
