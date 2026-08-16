//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 360/1335 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk360(t1302: f64, t493: f64, t511: f64, t544: f64, t1219: f64, t1220: f64, t1231: f64, t1236: f64, t1282: f64, t1286: f64, t1291: f64, t1293: f64, t1296: f64, t1300: f64, t267: f64) -> (f64, f64, f64) {
    let t1304 = 4.0_f64 / 15.0_f64 * t493 * t1302;
    let t1306 = 4.0_f64 / 15.0_f64 * t511 * t544;
    let t1307 = t1219 - 4.0_f64 / 45.0_f64 * t1220 - t1231 * t267 / 15.0_f64 - t1236 - t1282 + t1286 + t1291 - t1293 - t1296 + t1300 + t1304 - t1306;
    (t1304, t1306, t1307)
}
