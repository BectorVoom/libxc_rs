//! MGGA_C_RMGGAC lxc pol — lxc_pol part 15 (v4rho3sigma_6) CSE chunk 927/1110 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part15_v4rho3sigma_6_chunk927(t1614: f64, t3351: f64, t511: f64, t618: f64, t7231: f64, t10095: f64, t16043: f64, t1528: f64, t515: f64, t570: f64, t1652: f64, t39388: f64, t45403: f64, t45407: f64, t45411: f64, t45415: f64, t45420: f64, t45424: f64, t45428: f64, t45432: f64, t45436: f64, t45439: f64, t45441: f64, t45446: f64) -> f64 {
    let t45451 = t3351 * t7231 * t511 * t618 * t1614;
    let t45453 = t16043 * t10095;
    let t45458 = t3351 * t7231 * t515 * t1528 * t570;
    let t45463 = t3351 * t7231 * t515 * t618 * t1652;
    let t45465 = -0.1064114997332445985e-4_f64 * t45403 + 0.3192344991997337955e-4_f64 * t45407 - 0.3192344991997337955e-4_f64 * t45411 - 0.1064114997332445985e-4_f64 * t45415 + 0.29810146462873361018e-2_f64 * t39388 - 0.40911992481368012592e-1_f64 * t45420 - 0.212822999466489197e-4_f64 * t45424 - 0.17025839957319135759e-4_f64 * t45428 + 0.51077519871957407276e-4_f64 * t45432 - 0.17025839957319135759e-4_f64 * t45436 + 0.17025839957319135759e-4_f64 * t45439 - 0.31923449919973379548e-4_f64 * t45441 + 0.25538759935978703638e-4_f64 * t45446 + 0.25538759935978703638e-4_f64 * t45451 + 0.85129199786595678796e-5_f64 * t45453 + 0.85129199786595678796e-5_f64 * t45458 + 0.85129199786595678796e-5_f64 * t45463;
    t45465
}
