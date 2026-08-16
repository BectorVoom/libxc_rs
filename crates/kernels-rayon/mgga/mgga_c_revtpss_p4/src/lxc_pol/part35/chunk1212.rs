//! MGGA_C_REVTPSS lxc pol — lxc_pol part 35 (v4rho3sigma_10) CSE chunk 1212/1234 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part35_v4rho3sigma_10_chunk1212(t102015: f64, t114776: f64, t114780: f64, t114791: f64, t115074: f64, t115098: f64, t115126: f64, t115152: f64, t115181: f64, t115209: f64, t115238: f64, t115258: f64, t115352: f64, t115386: f64, t1450: f64, t1843: f64, t2014: f64, t2052: f64, t2089: f64, t22747: f64, t25043: f64, t25082: f64, t28196: f64, t28286: f64, t29498: f64, t29506: f64, t30122: f64, t30209: f64, t30314: f64, t30513: f64, t30553: f64, t30586: f64, t30614: f64, t30617: f64, t34495: f64, t4248: f64, t508: f64, t532: f64, t5542: f64, t569: f64, t5877: f64, t6765: f64, t7488: f64, t7898: f64, t7969: f64, t8065: f64, t8079: f64, t98450: f64) -> f64 {
    let t115406 = 3.0_f64 * t2014 * t7488 * t114776 + 6.0_f64 * t7898 * t30617 + 18.0_f64 * t7898 * t30614 + 9.0_f64 * t29506 * t8079 + 18.0_f64 * t25082 * t28286 * t114791 + t2014 * t532 * (t115074 + t115098 + t115126 + t115152 + t115181 + t115209 + t115238 + t115258) * t1450 - 18.0_f64 * t25082 * t34495 * t30122 - 3.0_f64 * t7969 * t6765 - t2052 * t25043 - t115352 * t508 - 3.0_f64 * t30553 * t1843 + t115386 * t569 + 6.0_f64 * t28196 * t28286 * t114780 - 3.0_f64 * t2014 * t30314 * t5542 + 18.0_f64 * t7898 * t30586 - t22747 * t2089 - 3.0_f64 * t5877 * t8065 - 18.0_f64 * t98450 * t30513 + 18.0_f64 * t2014 * t102015 * t29498 - 12.0_f64 * t4248 * t30209;
    t115406
}
