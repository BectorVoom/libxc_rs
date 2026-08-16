//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 20 (v4rho4_1) CSE chunk 2176/2712 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2176(t11521: f64, t1174: f64, t3431: f64, t1184: f64, t15394: f64, t11147: f64, t460: f64, t9288: f64, t11588: f64, t3469: f64, t3447: f64, t3451: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t44502 = t1174 * t3431 * t11521;
    let t44504 = t15394 * t1184;
    let t44505 = t460 * t11147;
    let t44506 = t44505 * t9288;
    let t44510 = t11588 * t3469;
    let t44512 = t3447 * t44510 * t3451;
    (t44502, t44504, t44505, t44506, t44510, t44512)
}
