//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 24 (v4rho3sigma_0) CSE chunk 1360/1438 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part24_v4rho3sigma_0_chunk1360(t10213: f64, t344: f64, t381: f64, t23384: f64, t23396: f64, t23326: f64, t6712: f64, t1054: f64, t2770: f64, t1049: f64, t225: f64, t23729: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t82390 = t10213 * t344;
    let t82391 = t82390 * t381;
    let t82400 = t23384 * t23396;
    let t82402 = t6712 * t23326;
    let t82411 = t1054 * t2770;
    let t82417 = t344 * t1049 * t225;
    let t82426 = t23384 * t23729;
    (t82390, t82391, t82400, t82402, t82411, t82417, t82426)
}
