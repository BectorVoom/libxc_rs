//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 20 (v4rho4_1) CSE chunk 2179/2712 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2179(t1176: f64, t697: f64, t1184: f64, t3447: f64, t3451: f64, t11579: f64, t11589: f64, t11168: f64, t15402: f64, t11159: f64, t15419: f64, t11584: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t44583 = t697 * t1176;
    let t44584 = t44583 * t1184;
    let t44586 = t3447 * t44584 * t3451;
    let t44589 = t3447 * t11589 * t11579;
    let t44592 = t3447 * t15402 * t11168;
    let t44595 = t3447 * t15419 * t11159;
    let t44602 = t3447 * t11589 * t11584;
    (t44583, t44584, t44586, t44589, t44592, t44595, t44602)
}
