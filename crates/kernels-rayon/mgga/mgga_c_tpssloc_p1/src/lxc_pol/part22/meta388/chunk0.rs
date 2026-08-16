//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 22 (v4rho4_3) CSE chunk 1656/2721 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1656(t1629: f64, t18107: f64, t1022: f64, t11060: f64, t5928: f64, t4684: f64, t5936: f64, t4673: f64, t1058: f64, t1061: f64, t11034: f64, t11037: f64, t11046: f64, t11059: f64, t11065: f64, t14618: f64, t14651: f64, t1630: f64, t18081: f64, t18083: f64, t18086: f64, t18089: f64, t18094: f64, t18100: f64, t18104: f64, t3180: f64, t3186: f64, t3200: f64, t4674: f64, t5929: f64, t5937: f64, t5939: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t18108 = t1629 * t18107;
    let t18111 = t11060 * t1022;
    let t18112 = t5928 * t18111;
    let t18117 = t5936 * t4684;
    let t18121 = t5936 * t4673;
    let t18124 = 2.0_f64 * t1058 * t18089 + t1058 * t18100 + t1061 * t18086 + 2.0_f64 * t11034 * t5929 - t11037 * t5939 + t11046 * t18094 + 6.0_f64 * t11059 * t18112 - 6.0_f64 * t11065 * t18104 + 4.0_f64 * t14618 * t4674 + 2.0_f64 * t14651 * t1630 - t18081 * t3200 + 2.0_f64 * t18083 * t3186 - 2.0_f64 * t18108 * t3200 - t18117 * t3200 + 2.0_f64 * t18121 * t3186 + t3180 * t5937;
    (t18108, t18111, t18112, t18117, t18121, t18124)
}
