//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 20 (v4rho4_1) CSE chunk 2571/2712 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2571(t14744: f64, t15402: f64, t3447: f64, t1174: f64, t135: f64, t15359: f64, t11589: f64, t15293: f64, t15382: f64, t44525: f64, t11588: f64, t4928: f64) -> (f64, f64, f64, f64, f64) {
    let t51995 = t3447 * t15402 * t14744;
    let t52013 = t1174 * t135 * t15359;
    let t52019 = t3447 * t11589 * t15293;
    let t52022 = t3447 * t44525 * t15382;
    let t52036 = t11588 * t4928;
    (t51995, t52013, t52019, t52022, t52036)
}
