//! MGGA_C_RMGGAC lxc pol — lxc_pol part 34 (v4rho2sigma2_7) CSE chunk 620/1097 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part34_v4rho2sigma2_7_chunk620(t118: f64, t15530: f64, t14461: f64, t14471: f64, t14505: f64, t15086: f64, t15089: f64, t15092: f64, t15140: f64, t15142: f64, t15146: f64, t15535: f64, t15538: f64, t15541: f64, t15544: f64, t15545: f64, t15546: f64, t15549: f64, t15550: f64, t15551: f64, t15552: f64) -> f64 {
    let t15557 = 0.39914139006212695214e-1_f64 * t118 * t15530;
    let t15558 = t15535 - t15538 + t15541 + t15086 - t15089 + t15092 + t15544 - t15545 + t15546 - t14461 + t14471 + t15549 - t14505 + t15550 - t15551 - t15552 - 0.93188427318671584245e-2_f64 * t15140 + 0.15531404553111930708e-1_f64 * t15142 + 0.31062809106223861415e-2_f64 * t15146 - t15557;
    t15558
}
