//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 53 (v4rho2sigma2_9) CSE chunk 889/1059 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part53_v4rho2sigma2_9_chunk889(t1352: f64, t32136: f64, t31153: f64, t31160: f64, t31177: f64, t31157: f64, t31163: f64, t31166: f64, t31173: f64, t31179: f64, t553: f64, t1332: f64, t1336: f64, t31621: f64, t31629: f64, t31633: f64, t32130: f64, t32132: f64, t544: f64, t8798: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t32137 = t32136 * t1352;
    let t32139 = 0.22608743412718618877e-1_f64 * t31153;
    let t32141 = 0.5383034145885385447e-3_f64 * t31160;
    let t32145 = 7.0_f64 / 576.0_f64 * t31177;
    let t32147 = -t32139 - 0.19378922925187387609e-1_f64 * t31157 - t32141 - 0.32298204875312312682e-2_f64 * t31163 + t31166 / 384.0_f64 - t31173 / 384.0_f64 - t32145 - t31179 / 96.0_f64;
    let t32148 = t553 * t32147;
    let t32150 = -t32130 - 0.6579736267392905746e-1_f64 * t31621 - t32132 - 0.3289868133696452873e-1_f64 * t31629 + 0.3289868133696452873e-1_f64 * t31633 + t1332 * t8798 - t1336 * t32137 + t544 * t32148;
    (t32137, t32139, t32141, t32145, t32147, t32148, t32150)
}
