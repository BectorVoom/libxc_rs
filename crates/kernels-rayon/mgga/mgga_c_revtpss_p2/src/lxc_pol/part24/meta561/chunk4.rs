//! MGGA_C_REVTPSS lxc pol — lxc_pol part 24 (v4rho4_4) CSE chunk 1689/1850 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1689(t5819: f64, t5825: f64, t6244: f64, t1042: f64, t1063: f64, t15696: f64, t15935: f64, t1651: f64, t1671: f64, t19738: f64, t19878: f64, t22671: f64, t23863: f64, t23899: f64, t23931: f64, t23939: f64, t3127: f64, t3161: f64, t3162: f64, t43082: f64, t4806: f64, t4837: f64, t4872: f64, t55141: f64, t65357: f64, t78561: f64, t78564: f64, t78576: f64, t78583: f64, t79038: f64, t88715: f64) -> (f64, f64, f64) {
    let t88732 = t5819 * t5825;
    let t88750 = t5819 * t6244;
    let t88763 = 0.38110238327173099531e-3_f64 * t78561 - 0.19055119163586549765e-2_f64 * t78564 - 0.11433071498151929859e-2_f64 * t78576 + 0.22866142996303859718e-2_f64 * t78583 + 0.51448821741683684366e-2_f64 * t1063 * t1042 * t15935 * t88732 + 0.85748036236139473944e-3_f64 * t79038 * t1671 - 0.57165357490759649296e-3_f64 * t3127 * t1042 * t4872 * t22671 * t1651 - 0.64311027177104605458e-3_f64 * t3161 * t1042 * t88715 * t3162 + 0.51448821741683684368e-2_f64 * t19738 * t23931 + 0.28582678745379824648e-2_f64 * t4837 * t1042 * t4806 * t88750 - 0.34299214494455789577e-2_f64 * t55141 * t23939 - 0.34299214494455789578e-2_f64 * t43082 * t15696 * t23899 - 0.19055119163586549765e-3_f64 * t65357 + 0.34299214494455789578e-2_f64 * t19878 * t23863;
    (t88732, t88750, t88763)
}
