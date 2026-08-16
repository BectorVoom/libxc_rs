//! MGGA_C_KCIS lxc pol — lxc_pol part 5 (v3rho3_2) CSE chunk 1315/1419 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part5_v3rho3_2_chunk1315(t21684: f64, t21719: f64, t21761: f64, t21784: f64, t532: f64, t7116: f64, t12084: f64, t12085: f64, t12087: f64, t1360: f64, t1455: f64, t17019: f64, t17045: f64, t17047: f64, t17062: f64, t17065: f64, t17248: f64, t17250: f64, t21453: f64, t21631: f64, t21633: f64, t21635: f64, t21637: f64, t21641: f64, t486: f64, t538: f64, t7028: f64, t7190: f64) -> (f64, f64) {
    let t21786 = t21684 + t21719 + t21761 + t21784;
    let t21788 = t532 * t7116;
    let t21790 = -0.46853067927761790996e-2_f64 * t12085 - 0.93706135855523581992e-2_f64 * t12087 - t17045 - t17047 - t17062 - t17065 - t12084 - 0.46853067927761790996e-2_f64 * t21631 + 0.23426533963880895498e-2_f64 * t21633 - 0.14055920378328537299e-1_f64 * t21635 + 0.46853067927761790996e-2_f64 * t21637 - t7028 * t1455 - t21453 * t538 - 0.28111840756657074598e-1_f64 * t17019 * t21641 - 0.93706135855523581992e-2_f64 * t17248 - 0.18741227171104716398e-1_f64 * t17250 - t1360 * t7190 - t486 * t21786 - 0.93706135855523581992e-2_f64 * t21788;
    (t21786, t21790)
}
