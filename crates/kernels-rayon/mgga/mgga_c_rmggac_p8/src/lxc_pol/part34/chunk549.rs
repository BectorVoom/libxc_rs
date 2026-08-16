//! MGGA_C_RMGGAC lxc pol — lxc_pol part 34 (v4rho2sigma2_7) CSE chunk 549/1097 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part34_v4rho2sigma2_7_chunk549(t2211: f64, t7799: f64, t739: f64, t7879: f64, t884: f64, t13957: f64, t8041: f64, t1356: f64, t13844: f64, t14276: f64, t14278: f64, t14280: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t14421 = t2211 * t7799;
    let t14422 = t739 * t14421;
    let t14423 = 0.11974241701863808564e0_f64 * t14422;
    let t14424 = t2211 * t7879;
    let t14425 = t884 * t14424;
    let t14426 = 0.11974241701863808564e0_f64 * t14425;
    let t14427 = t8041 * t13957;
    let t14428 = t1356 * t14427;
    let t14429 = 0.11974241701863808564e0_f64 * t14428;
    let t14430 = 0.31062809106223861415e-2_f64 * t13844;
    let t14431 = 0.20455996240684006298e-1_f64 * t14276;
    let t14432 = 0.2727466165424534173e-1_f64 * t14278;
    let t14433 = 0.13637330827122670865e-1_f64 * t14280;
    (t14421, t14423, t14424, t14426, t14427, t14429, t14430, t14431, t14432, t14433)
}
