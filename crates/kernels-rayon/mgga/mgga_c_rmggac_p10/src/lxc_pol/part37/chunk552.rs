//! MGGA_C_RMGGAC lxc pol — lxc_pol part 37 (v4rho2sigma2_10) CSE chunk 552/1128 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part37_v4rho2sigma2_10_chunk552(t14451: f64, t321: f64, t5259: f64, t333: f64, t4669: f64, t352: f64, t5148: f64, t14288: f64, t14291: f64, t14294: f64, t14299: f64, t14316: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t14452 = t14451 * t321;
    let t14453 = t5259 * t14452;
    let t14454 = 0.2993560425465952141e-1_f64 * t14453;
    let t14455 = t14451 * t333;
    let t14456 = t4669 * t14455;
    let t14457 = 0.44903406381989282115e-1_f64 * t14456;
    let t14458 = t14451 * t352;
    let t14459 = t5148 * t14458;
    let t14460 = 0.2993560425465952141e-1_f64 * t14459;
    let t14461 = 0.18183107769496894487e-1_f64 * t14288;
    let t14462 = 0.20455996240684006296e-1_f64 * t14291;
    let t14463 = 0.40911992481368012592e-1_f64 * t14294;
    let t14464 = 0.10227998120342003148e-1_f64 * t14299;
    let t14468 = 0.68186654135613354325e-2_f64 * t14316;
    (t14454, t14457, t14460, t14461, t14462, t14463, t14464, t14468)
}
