//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 22 (v4rho4_3) CSE chunk 2463/2721 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2463(t225: f64, t69840: f64, t1049: f64, t21390: f64, t1058: f64, t1060: f64, t11034: f64, t11059: f64, t11060: f64, t11065: f64, t14608: f64, t1632: f64, t17876: f64, t18103: f64, t18117: f64, t18131: f64, t21617: f64, t21643: f64, t21644: f64, t21647: f64, t21650: f64, t3186: f64, t3188: f64, t3200: f64, t43470: f64, t43473: f64, t4649: f64, t4684: f64, t5914: f64, t69996: f64) -> (f64, f64, f64) {
    let t70012 = t69840 * t225;
    let t70014 = t1049 * t21390;
    let t70068 = 3.0_f64 * t1058 * t1060 * t4649 * t5914 + 6.0_f64 * t11059 * t11060 * t70014 - 18.0_f64 * t11065 * t18103 * t21643 - 3.0_f64 * t21617 * t3200 * t4684 + 6.0_f64 * t3186 * t3188 * t69996 + 6.0_f64 * t11034 * t21644 - 3.0_f64 * t14608 * t18117 - 6.0_f64 * t14608 * t18131 + 3.0_f64 * t1632 * t17876 + 6.0_f64 * t21647 * t43473 - 6.0_f64 * t21650 * t43470;
    (t70012, t70014, t70068)
}
