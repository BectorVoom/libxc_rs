//! MGGA_C_RMGGAC lxc pol — lxc_pol part 13 (v4rho3sigma_4) CSE chunk 932/988 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part13_v4rho3sigma_4_chunk932<F: Float>(t40045: F, t798: F, t9523: F, t1364: F, t2211: F, t2471: F, t26287: F, t31043: F, t37764: F, t40032: F, t40037: F, t40039: F, t40043: F, t40047: F, t40050: F, t40055: F, t40057: F, t40060: F, t4048: F, t5187: F, t5194: F, t699: F, t739: F, t884: F, t903: F, t9530: F) -> (F, F) {
    let t43241 = 0.11918087970123395032e-3 * t40045;
    let t43261 = t9523 * t798;
    let t43266 = -0.5107751987195740728e-4 * t40032 - 0.638468998399467591e-4 * t40037 - 0.13637330827122670865e-1 * t40039 + 0.5107751987195740728e-4 * t40043 - t43241 + 0.5107751987195740728e-4 * t40047 + 0.8980681276397856423e-1 * t40050 + 0.35922725105591425692e0 * t903 * t2471 * t798 + 0.23948483403727617128e0 * t739 * t9530 * t4048 + 0.35922725105591425692e0 * t903 * t699 * t5187 - 0.47896966807455234256e0 * t1364 * t699 * t5194 - 0.23948483403727617128e0 * t884 * t2211 * t31043 + 0.5107751987195740728e-4 * t40055 - 0.4726e1 * t37764 + 0.71845450211182851384e0 * t26287 * t43261 - 0.16364796992547205038e0 * t40057 + 0.2727466165424534173e0 * t40060;
    (t43261, t43266)
}
