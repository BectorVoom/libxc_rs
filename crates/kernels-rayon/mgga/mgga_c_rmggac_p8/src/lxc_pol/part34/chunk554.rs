//! MGGA_C_RMGGAC lxc pol — lxc_pol part 34 (v4rho2sigma2_7) CSE chunk 554/1097 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part34_v4rho2sigma2_7_chunk554(t265: f64, t699: f64, t305: f64, t118: f64, t14303: f64, t14306: f64, t14312: f64, t14431: f64, t14432: f64, t14433: f64, t14435: f64, t14440: f64, t14443: f64, t14447: f64, t14450: f64, t14454: f64, t14457: f64, t14460: f64, t14461: f64, t14462: f64, t14463: f64, t14464: f64, t14468: f64) -> (f64, f64, f64) {
    let t14469 = t699 * t265;
    let t14470 = t305 * t14469;
    let t14471 = 0.39914139006212695213e-1_f64 * t14470;
    let t14472 = t14431 - t14432 - t14433 - 0.39914139006212695214e-1_f64 * t118 * t14435 - t14440 - t14443 + t14447 - t14450 - t14454 + t14457 + t14460 - t14461 + t14462 - t14463 - t14464 - 0.93188427318671584245e-2_f64 * t14303 + 0.15531404553111930708e-1_f64 * t14306 + 0.31062809106223861415e-2_f64 * t14312 + t14468 + t14471;
    (t14469, t14471, t14472)
}
