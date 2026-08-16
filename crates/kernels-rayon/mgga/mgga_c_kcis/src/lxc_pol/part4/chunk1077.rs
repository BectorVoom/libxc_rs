//! MGGA_C_KCIS lxc pol — lxc_pol part 4 (v3rho3_1) CSE chunk 1077/1420 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part4_v3rho3_1_chunk1077(t1717: f64, t2630: f64, t1646: f64, t3073: f64, t3074: f64, t1728: f64, t10170: f64, t1727: f64, t1030: f64, t829: f64, t1045: f64, t10093: f64, t10150: f64, t10182: f64, t10188: f64, t10190: f64, t10192: f64, t10194: f64, t1083: f64, t13618: f64, t13620: f64, t13623: f64, t13627: f64, t13630: f64, t1697: f64, t278: f64, t305: f64, t3061: f64, t3166: f64, t4768: f64, t4920: f64, t975: f64) -> f64 {
    let t13633 = t1717 * t2630;
    let t13636 = t3073 * t1646;
    let t13637 = t13636 * t3074;
    let t13640 = t1728 * t2630;
    let t13643 = t10170 * t1727;
    let t13644 = t13643 * t3074;
    let t13658 = t1030 * t3073;
    let t13659 = t1727 * t829;
    let t13660 = t13659 * t1045;
    let t13663 = -t278 * t13618 + 0.93706135855523581992e-2_f64 * t1030 * t13620 + 0.46853067927761790996e-2_f64 * t1030 * t13623 + 0.28111840756657074598e-1_f64 * t305 * t13627 + 0.14055920378328537299e-1_f64 * t305 * t13630 - 0.14055920378328537299e-1_f64 * t10093 * t13633 - 0.14055920378328537299e-1_f64 * t1030 * t13637 - 0.93706135855523581992e-2_f64 * t3061 * t13640 - 0.56223681513314149196e-1_f64 * t305 * t13644 - 2.0_f64 * t4768 * t1083 - t1697 * t3166 - 2.0_f64 * t975 * t4920 - 0.14055920378328537299e-1_f64 * t10150 - 0.46853067927761790996e-2_f64 * t10182 - 0.93706135855523581992e-2_f64 * t10188 - 0.18741227171104716398e-1_f64 * t10190 + 0.23426533963880895498e-2_f64 * t10192 + 0.46853067927761790996e-2_f64 * t10194 - 0.28111840756657074598e-1_f64 * t13658 * t13660;
    t13663
}
