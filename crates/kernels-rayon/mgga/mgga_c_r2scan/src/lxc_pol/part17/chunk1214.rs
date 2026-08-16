//! MGGA_C_R2SCAN lxc pol — lxc_pol part 17 (v4rho3sigma_7) CSE chunk 1214/1293 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_r2scan_lxc_pol_part17_v4rho3sigma_7_chunk1214(t11338: f64, t12422: f64, t11518: f64, t12098: f64, t3262: f64, t11345: f64, t12567: f64, t11523: f64, t12086: f64, t11199: f64, t12570: f64, t3275: f64, t3472: f64, t42901: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t44122 = t12422 * t11338 / 4.0_f64;
    let t44125 = 15.0_f64 / 8.0_f64 * t3262 * t12098 * t11518;
    let t44127 = t12567 * t11345 / 4.0_f64;
    let t44129 = t11523 * t12086 / 2.0_f64;
    let t44132 = 3.0_f64 / 4.0_f64 * t3262 * t11199 * t12570;
    let t44135 = 5.0_f64 / 16.0_f64 * t3275 * t3472 * t42901;
    (t44122, t44125, t44127, t44129, t44132, t44135)
}
