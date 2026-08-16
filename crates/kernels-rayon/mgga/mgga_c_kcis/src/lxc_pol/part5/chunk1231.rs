//! MGGA_C_KCIS lxc pol — lxc_pol part 5 (v3rho3_2) CSE chunk 1231/1419 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part5_v3rho3_2_chunk1231(t3530: f64, t6837: f64, t1262: f64, t5329: f64, t3500: f64, t6770: f64, t1251: f64, t15518: f64, t15547: f64, t15549: f64, t20601: f64, t20604: f64, t20607: f64, t20610: f64, t20614: f64, t3490: f64, t3514: f64, t6759: f64, t6771: f64) -> f64 {
    let t20617 = t3530 * t6837;
    let t20618 = t20617 * t1262;
    let t20619 = t5329 * t20618;
    let t20624 = t3500 * t6770;
    let t20625 = t1251 * t20624;
    let t20630 = -t3514 * t20601 / 432.0_f64 - t3514 * t20604 / 72.0_f64 + 7.0_f64 / 1296.0_f64 * t3514 * t20607 + t3514 * t20610 / 108.0_f64 - t3514 * t20614 / 288.0_f64 - t15518 + t1251 * t20619 / 96.0_f64 - t3490 * t6771 / 216.0_f64 + t20625 / 1728.0_f64 - t3490 * t6759 / 162.0_f64 - t15547 - t15549 / 1296.0_f64;
    t20630
}
