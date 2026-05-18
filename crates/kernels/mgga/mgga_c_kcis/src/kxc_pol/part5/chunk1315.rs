//! MGGA_C_KCIS kxc pol — kxc_pol part 5 (v3rho3_2) CSE chunk 1315/1419 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_kxc_pol_part5_v3rho3_2_chunk1315<F: Float>(t21684: F, t21719: F, t21761: F, t21784: F, t532: F, t7116: F, t12084: F, t12085: F, t12087: F, t1360: F, t1455: F, t17019: F, t17045: F, t17047: F, t17062: F, t17065: F, t17248: F, t17250: F, t21453: F, t21631: F, t21633: F, t21635: F, t21637: F, t21641: F, t486: F, t538: F, t7028: F, t7190: F) -> (F, F) {
    let t21786 = t21684 + t21719 + t21761 + t21784;
    let t21788 = t532 * t7116;
    let t21790 = -F::new(0.46853067927761790996e-2) * t12085 - F::new(0.93706135855523581992e-2) * t12087 - t17045 - t17047 - t17062 - t17065 - t12084 - F::new(0.46853067927761790996e-2) * t21631 + F::new(0.23426533963880895498e-2) * t21633 - F::new(0.14055920378328537299e-1) * t21635 + F::new(0.46853067927761790996e-2) * t21637 - t7028 * t1455 - t21453 * t538 - F::new(0.28111840756657074598e-1) * t17019 * t21641 - F::new(0.93706135855523581992e-2) * t17248 - F::new(0.18741227171104716398e-1) * t17250 - t1360 * t7190 - t486 * t21786 - F::new(0.93706135855523581992e-2) * t21788;
    (t21786, t21790)
}
