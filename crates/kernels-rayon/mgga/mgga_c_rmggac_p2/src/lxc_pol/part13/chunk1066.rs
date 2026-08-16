//! MGGA_C_RMGGAC lxc pol — lxc_pol part 13 (v4rho3sigma_4) CSE chunk 1066/1127 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part13_v4rho3sigma_4_chunk1066(t40045: f64, t798: f64, t9523: f64, t1364: f64, t2211: f64, t2471: f64, t26287: f64, t31043: f64, t37764: f64, t40032: f64, t40037: f64, t40039: f64, t40043: f64, t40047: f64, t40050: f64, t40055: f64, t40057: f64, t40060: f64, t4048: f64, t5187: f64, t5194: f64, t699: f64, t739: f64, t884: f64, t903: f64, t9530: f64) -> (f64, f64) {
    let t43241 = 0.11918087970123395032e-3_f64 * t40045;
    let t43261 = t9523 * t798;
    let t43266 = -0.5107751987195740728e-4_f64 * t40032 - 0.638468998399467591e-4_f64 * t40037 - 0.13637330827122670865e-1_f64 * t40039 + 0.5107751987195740728e-4_f64 * t40043 - t43241 + 0.5107751987195740728e-4_f64 * t40047 + 0.8980681276397856423e-1_f64 * t40050 + 0.35922725105591425692e0_f64 * t903 * t2471 * t798 + 0.23948483403727617128e0_f64 * t739 * t9530 * t4048 + 0.35922725105591425692e0_f64 * t903 * t699 * t5187 - 0.47896966807455234256e0_f64 * t1364 * t699 * t5194 - 0.23948483403727617128e0_f64 * t884 * t2211 * t31043 + 0.5107751987195740728e-4_f64 * t40055 - 0.4726e1_f64 * t37764 + 0.71845450211182851384e0_f64 * t26287 * t43261 - 0.16364796992547205038e0_f64 * t40057 + 0.2727466165424534173e0_f64 * t40060;
    (t43261, t43266)
}
