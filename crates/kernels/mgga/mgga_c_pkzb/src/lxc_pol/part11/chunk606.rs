//! MGGA_C_PKZB lxc pol — lxc_pol part 11 (v4rho4_3) CSE chunk 606/1208 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part11_v4rho4_3_chunk606<F: Float>(t3487: F, t626: F, t1045: F, t1055: F, t184: F, t188: F, t3461: F, t3467: F, t1020: F, t2714: F, t1058: F, t135: F, t144: F, t1501: F, t1510: F, t1520: F, t1530: F, t1534: F, t1535: F, t1544: F, t1633: F, t1676: F, t3382: F, t3396: F, t3401: F, t3422: F, t3427: F, t560: F, t639: F) -> (F, F, F, F) {
    let t3488 = t626 * t3487;
    let t3491 = 0.65854491829355115987e0 * t3461 * t188 - 0.13170898365871023197e1 * t1045 * t1055 + 0.13170898365871023197e1 * t184 * t3467 - 0.65854491829355115987e0 * t184 * t3488;
    let t3495 = t2714 * t1020;
    let t3501 = t1058 * t1058;
    let t3505 = -t135 * t144 * t1676 * t3501 + t135 * t144 * t3491 * t639 + 6.0 * t135 * t1633 * t3401 + 3.0 * t135 * t3396 * t560 + 6.0 * t1535 * t3495 - t1501 - t1510 - t1520 + t1530 + t1534 + t1544 + t3382 + t3422 + t3427;
    (t3488, t3491, t3501, t3505)
}
