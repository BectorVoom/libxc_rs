//! MGGA_C_R2SCAN lxc pol — lxc_pol part 17 (v4rho3sigma_7) CSE chunk 826/1293 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_r2scan_lxc_pol_part17_v4rho3sigma_7_chunk826(t113: f64, t8691: f64, t3052: f64, t494: f64, t2530: f64, t285: f64, t3055: f64, t3270: f64, t983: f64, t1561: f64, t3060: f64, t3229: f64, t498: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t8692 = t8691 * t113;
    let t8694 = t3052 * t494;
    let t8698 = t285 * t2530;
    let t8701 = t3055 * t494;
    let t8707 = t3270 * t983;
    let t8714 = t1561 * t3060;
    let t8723 = t498 * t3229;
    (t8692, t8694, t8698, t8701, t8707, t8714, t8723)
}
