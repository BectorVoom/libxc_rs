//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 21 (v4rho4_2) CSE chunk 3102/3221 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk3102(t43959: f64, t6024: f64, t1128: f64, t18668: f64, t3263: f64, t5983: f64, t3266: f64, t1129: f64, t1137: f64, t1138: f64, t11410: f64, t15118: f64, t15141: f64, t1683: f64, t18840: f64, t18894: f64, t3327: f64, t3352: f64, t3360: f64, t44211: f64, t4797: f64, t4820: f64, t51594: f64, t6037: f64, t6053: f64, t63763: f64, t63765: f64, t63767: f64, t63769: f64, t63771: f64, t63829: f64, t64100: f64, t64103: f64, t64132: f64, t64148: f64, t64165: f64, t64181: f64, t64197: f64, t64212: f64, t64229: f64, t64245: f64) -> (f64, f64, f64) {
    let t64253 = 0.16081979498692535067e2_f64 * t43959 * t6024;
    let t64254 = t18668 * t1128;
    let t64257 = t5983 * t3263;
    let t64259 = 2.0_f64 * t64257 * t3266;
    let t64260 = -t63763 - t63765 + t63767 - t63769 - t63771 + t63829 - t64100 + 1.0_f64 * t18840 * t3352 + 0.32163958997385070134e2_f64 * t64103 * t3360 + 2.0_f64 * t51594 * t1683 + 4.0_f64 * t15141 * t4820 + 2.0_f64 * t4797 * t15118 - 2.0_f64 * t44211 * t6037 + 1.0_f64 * t11410 * t6053 + 2.0_f64 * t3327 * t18894 + 1.0_f64 * t1129 * (t64132 + t64148 + t64165 + t64181 + t64197 + t64212 + t64229 + t64245) * t1137 - t64253 + 2.0_f64 * t64254 * t1138 + t64259;
    (t64253, t64259, t64260)
}
