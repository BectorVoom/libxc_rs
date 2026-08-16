//! MGGA_C_PKZB lxc pol — lxc_pol part 11 (v4rho4_3) CSE chunk 623/1340 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_pkzb_lxc_pol_part11_v4rho4_3_chunk623(t3487: f64, t626: f64, t1045: f64, t1055: f64, t184: f64, t188: f64, t3461: f64, t3467: f64, t1020: f64, t2714: f64, t1058: f64, t135: f64, t144: f64, t1501: f64, t1510: f64, t1520: f64, t1530: f64, t1534: f64, t1535: f64, t1544: f64, t1633: f64, t1676: f64, t3382: f64, t3396: f64, t3401: f64, t3422: f64, t3427: f64, t560: f64, t639: f64) -> (f64, f64, f64, f64) {
    let t3488 = t626 * t3487;
    let t3491 = 0.65854491829355115987e0_f64 * t3461 * t188 - 0.13170898365871023197e1_f64 * t1045 * t1055 + 0.13170898365871023197e1_f64 * t184 * t3467 - 0.65854491829355115987e0_f64 * t184 * t3488;
    let t3495 = t2714 * t1020;
    let t3501 = t1058 * t1058;
    let t3505 = -t135 * t144 * t1676 * t3501 + t135 * t144 * t3491 * t639 + 6.0_f64 * t135 * t1633 * t3401 + 3.0_f64 * t135 * t3396 * t560 + 6.0_f64 * t1535 * t3495 - t1501 - t1510 - t1520 + t1530 + t1534 + t1544 + t3382 + t3422 + t3427;
    (t3488, t3491, t3501, t3505)
}
