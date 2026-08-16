//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 20 (v4rho4_1) CSE chunk 1928/2712 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1928(t11539: f64, t4724: f64, t1174: f64, t15239: f64, t475: f64, t1214: f64, t248: f64, t3494: f64, t4977: f64, t4582: f64, t3516: f64, t12652: f64, t4987: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t15522 = t11539 * t4724;
    let t15524 = t1174 * t15522 / 324.0_f64;
    let t15525 = t15239 * t475;
    let t15527 = t248 * t1214 * t15525;
    let t15530 = t4977 * t3494;
    let t15531 = t4582 * t15530;
    let t15534 = t4977 * t3516;
    let t15535 = t4582 * t15534;
    let t15540 = t4987 * t12652;
    (t15524, t15525, t15527, t15530, t15531, t15534, t15535, t15540)
}
