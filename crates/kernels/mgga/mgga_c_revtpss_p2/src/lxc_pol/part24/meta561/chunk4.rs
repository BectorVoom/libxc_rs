//! MGGA_C_REVTPSS lxc pol — lxc_pol part 24 (v4rho4_4) CSE chunk 1689/1850 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1689<F: Float>(t5819: F, t5825: F, t6244: F, t1042: F, t1063: F, t15696: F, t15935: F, t1651: F, t1671: F, t19738: F, t19878: F, t22671: F, t23863: F, t23899: F, t23931: F, t23939: F, t3127: F, t3161: F, t3162: F, t43082: F, t4806: F, t4837: F, t4872: F, t55141: F, t65357: F, t78561: F, t78564: F, t78576: F, t78583: F, t79038: F, t88715: F) -> (F, F, F) {
    let t88732 = t5819 * t5825;
    let t88750 = t5819 * t6244;
    let t88763 = F::cast_from(0.38110238327173099531e-3_f64) * t78561 - F::cast_from(0.19055119163586549765e-2_f64) * t78564 - F::cast_from(0.11433071498151929859e-2_f64) * t78576 + F::cast_from(0.22866142996303859718e-2_f64) * t78583 + F::cast_from(0.51448821741683684366e-2_f64) * t1063 * t1042 * t15935 * t88732 + F::cast_from(0.85748036236139473944e-3_f64) * t79038 * t1671 - F::cast_from(0.57165357490759649296e-3_f64) * t3127 * t1042 * t4872 * t22671 * t1651 - F::cast_from(0.64311027177104605458e-3_f64) * t3161 * t1042 * t88715 * t3162 + F::cast_from(0.51448821741683684368e-2_f64) * t19738 * t23931 + F::cast_from(0.28582678745379824648e-2_f64) * t4837 * t1042 * t4806 * t88750 - F::cast_from(0.34299214494455789577e-2_f64) * t55141 * t23939 - F::cast_from(0.34299214494455789578e-2_f64) * t43082 * t15696 * t23899 - F::cast_from(0.19055119163586549765e-3_f64) * t65357 + F::cast_from(0.34299214494455789578e-2_f64) * t19878 * t23863;
    (t88732, t88750, t88763)
}
