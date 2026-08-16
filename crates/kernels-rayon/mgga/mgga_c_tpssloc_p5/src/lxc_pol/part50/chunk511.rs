//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 50 (v4rho2sigma2_6) CSE chunk 511/1294 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part50_v4rho2sigma2_6_chunk511(t1887: f64, t3446: f64, t1176: f64, t60: f64, t1184: f64, t1089: f64, t460: f64, t607: f64, t3293: f64, t1191: f64, t225: f64, t1202: f64, t1226: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t3447 = t3446 * t1887;
    let t3448 = t60 * t1176;
    let t3449 = t3448 * t1184;
    let t3450 = t460 * t1089;
    let t3451 = t3450 * t607;
    let t3464 = 5.0_f64 / 18.0_f64 * t3293;
    let t3487 = t1191 * t225;
    let t3490 = t1202 * t1226;
    (t3447, t3448, t3449, t3450, t3451, t3464, t3487, t3490)
}
