//! MGGA_C_R2SCAN lxc pol — lxc_pol part 18 (v4rho3sigma_8) CSE chunk 1010/1264 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_r2scan_lxc_pol_part18_v4rho3sigma_8_chunk1010(t3308: f64, t9296: f64, t1577: f64, t2651: f64, t3597: f64, t9292: f64, t574: f64, t9445: f64, t2124: f64, t9422: f64, t3295: f64, t9376: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t12511 = t3308 * t9296;
    let t12512 = t1577 * t12511;
    let t12515 = t2651 * t3597;
    let t12517 = t3308 * t9292;
    let t12518 = t574 * t12517;
    let t12520 = t3308 * t9445;
    let t12521 = t574 * t12520;
    let t12523 = t2124 * t9422;
    let t12524 = t3295 * t12523;
    let t12526 = t2124 * t9376;
    (t12511, t12512, t12515, t12517, t12518, t12520, t12521, t12523, t12524, t12526)
}
