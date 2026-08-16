//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 22 (v4rho4_3) CSE chunk 2457/2721 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2457(t21682: f64, t225: f64, t1009: f64, t21480: f64, t1057: f64, t10482: f64, t5866: f64, t1022: f64, t1049: f64, t1058: f64, t1060: f64, t1061: f64, t11059: f64, t14618: f64, t18083: f64, t18100: f64, t18111: f64, t18138: f64, t18162: f64, t21594: f64, t21618: f64, t21637: f64, t21643: f64, t23508: f64, t3180: f64, t3186: f64, t360: f64, t43503: f64, t43576: f64, t43577: f64, t4669: f64, t50508: f64, t50509: f64, t5932: f64) -> (f64, f64, f64, f64) {
    let t69871 = t21682 * t225;
    let t69923 = t21480 * t1009;
    let t69924 = t69923 * t1057;
    let t69935 = t10482 * t5866;
    let t69942 = -t1022 * t21637 * t23508 * t360 * t43503 + 24.0_f64 * t1022 * t21637 * t43576 * t43577 + 18.0_f64 * t1022 * t50508 * t50509 * t69935 + t1049 * t1058 * t1060 * t21594 + 18.0_f64 * t11059 * t18111 * t21643 + 12.0_f64 * t18138 * t3186 * t5932 + t1061 * t69924 + 6.0_f64 * t14618 * t18083 + 3.0_f64 * t18100 * t4669 + 3.0_f64 * t18162 * t4669 + 3.0_f64 * t21618 * t3180;
    (t69871, t69923, t69935, t69942)
}
