//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 21 (v4rho4_2) CSE chunk 2968/3221 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2968(t13969: f64, t17713: f64, t3130: f64, t4649: f64, t884: f64, t1023: f64, t10390: f64, t10403: f64, t10408: f64, t1041: f64, t14211: f64, t17187: f64, t17688: f64, t17972: f64, t18021: f64, t3048: f64, t3070: f64, t3071: f64, t3121: f64, t3132: f64, t4579: f64, t4582: f64, t47775: f64, t48626: f64, t48629: f64, t48670: f64, t48674: f64, t50324: f64, t5677: f64, t61853: f64, t61855: f64) -> (f64, f64) {
    let t61866 = t3130 * t13969 * t17713;
    let t61871 = t884 * t4649;
    let t61876 = t50324 * t4579 / 1152.0_f64 + t10390 * t18021 / 2304.0_f64 + t3070 * t3071 * t17187 * t1023 / 2304.0_f64 + 5.0_f64 / 13824.0_f64 * t3070 * t10408 * t5677 * t3121 + 5.0_f64 / 6912.0_f64 * t10403 * t10408 * t5677 * t3132 + t61853 / 576.0_f64 - t1041 * t4582 * t47775 * t61855 / 192.0_f64 + 5.0_f64 / 216.0_f64 * t3048 * t17688 - t48626 / 864.0_f64 + 5.0_f64 / 5184.0_f64 * t48629 + t48670 / 5184.0_f64 + t61866 / 1152.0_f64 - t3048 * t17972 / 72.0_f64 + t48674 / 7776.0_f64 + t10403 * t3071 * t14211 * t61871 / 576.0_f64;
    (t61871, t61876)
}
