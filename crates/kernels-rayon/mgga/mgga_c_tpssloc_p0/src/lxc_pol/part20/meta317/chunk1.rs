//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 20 (v4rho4_1) CSE chunk 1579/2712 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1579(t11539: f64, t3442: f64, t1174: f64, t11499: f64, t11505: f64, t11510: f64, t11514: f64, t11518: f64, t11522: f64, t11526: f64, t11531: f64, t11534: f64, t11537: f64) -> (f64, f64, f64) {
    let t11540 = t11539 * t3442;
    let t11541 = t1174 * t11540;
    let t11543 = -0.83333333333333333332e-3_f64 * t1174 * t11499 - 0.83333333333333333332e-3_f64 * t1174 * t11505 - 0.24999999999999999999e-2_f64 * t1174 * t11510 - 0.83333333333333333331e-3_f64 * t11514 + 0.22222222222222222221e-2_f64 * t1174 * t11518 - 0.16666666666666666666e-2_f64 * t1174 * t11522 - 0.27777777777777777777e-3_f64 * t1174 * t11526 + 0.18518518518518518518e-3_f64 * t11531 - 0.27777777777777777777e-3_f64 * t11534 - 0.55555555555555555554e-3_f64 * t11537 + 0.37037037037037037036e-3_f64 * t11541;
    (t11540, t11541, t11543)
}
