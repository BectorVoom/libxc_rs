//! MGGA_C_R2SCAN lxc pol — lxc_pol part 15 (v4rho3sigma_5) CSE chunk 1243/1253 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_r2scan_lxc_pol_part15_v4rho3sigma_5_chunk1243(t1020: f64, t1083: f64, t1085: f64, t11106: f64, t11108: f64, t11111: f64, t11113: f64, t11981: f64, t11983: f64, t11985: f64, t11987: f64, t11989: f64, t1310: f64, t2410: f64, t3388: f64, t3390: f64, t3394: f64, t3652: f64, t839: f64, t8438: f64) -> f64 {
    let t40986 = -0.4355305902528e1_f64 * t11989 * t839 - 0.18428227254588e2_f64 * t11981 * t839 + 0.734774460522e2_f64 * t11983 * t839 - 0.7662840944824e2_f64 * t11985 * t839 + 0.3101306810232e2_f64 * t11987 * t839 - 0.9214113627294e1_f64 * t11106 * t1020 - 0.18428227254588e2_f64 * t11108 * t1020 - 0.18428227254588e2_f64 * t3388 * t2410 - 0.9214113627294e1_f64 * t11111 * t1020 - 0.18428227254588e2_f64 * t3390 * t2410 - 0.9214113627294e1_f64 * t1083 * t8438 - 0.9214113627294e1_f64 * t3652 * t1310 + 0.367387230261e2_f64 * t11113 * t1020 + 0.734774460522e2_f64 * t3394 * t2410 + 0.367387230261e2_f64 * t1085 * t8438;
    t40986
}
