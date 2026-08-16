//! MGGA_C_PKZB lxc pol — lxc_pol part 9 (v4rho4_1) CSE chunk 1057/1336 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_pkzb_lxc_pol_part9_v4rho4_1_chunk1057(t1597: f64, t1517: f64, t1600: f64, t57: f64, t1531: f64, t4902: f64, t557: f64, t4865: f64, t4871: f64, t466: f64, t5342: f64, t5089: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t16518 = t1597 * t1597;
    let t16521 = t1517 * t1517;
    let t16522 = t1600 * t1600;
    let t16526 = 0.24955700379505800916e5_f64 * t57 / t16518 * t16521 / t16522;
    let t16531 = 0.67471172535210825684e-1_f64 * t1531 * t4902 * t557;
    let t16532 = t4871 * t4865;
    let t16536 = 0.21687162600603479684e-1_f64 * t1531 * t466 * t5342;
    let t16539 = 0.38527786510141256862e1_f64 * t1531 * t466 * t5089;
    (t16521, t16526, t16531, t16532, t16536, t16539)
}
