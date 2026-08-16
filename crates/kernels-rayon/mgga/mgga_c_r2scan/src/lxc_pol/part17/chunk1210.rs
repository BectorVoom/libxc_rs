//! MGGA_C_R2SCAN lxc pol — lxc_pol part 17 (v4rho3sigma_7) CSE chunk 1210/1293 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_r2scan_lxc_pol_part17_v4rho3sigma_7_chunk1210(t3262: f64, t3465: f64, t43950: f64, t12812: f64, t3270: f64, t3269: f64, t3275: f64, t3472: f64, t42424: f64, t11189: f64, t42419: f64, t43798: f64) -> (f64, f64, f64, f64, f64) {
    let t44077 = 3.0_f64 / 4.0_f64 * t3262 * t3465 * t43950;
    let t44078 = t3270 * t12812;
    let t44080 = t3269 * t44078 / 4.0_f64;
    let t44083 = 5.0_f64 / 16.0_f64 * t3275 * t3472 * t42424;
    let t44086 = 45.0_f64 / 64.0_f64 * t3275 * t11189 * t42419;
    let t44089 = 5.0_f64 / 8.0_f64 * t3275 * t3472 * t43798;
    (t44077, t44080, t44083, t44086, t44089)
}
