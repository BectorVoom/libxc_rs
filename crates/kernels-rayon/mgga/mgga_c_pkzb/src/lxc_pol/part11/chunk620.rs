//! MGGA_C_PKZB lxc pol — lxc_pol part 11 (v4rho4_3) CSE chunk 620/1340 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_pkzb_lxc_pol_part11_v4rho4_3_chunk620(t158: f64, t3428: f64, t3429: f64, t1746: f64, t3401: f64, t3396: f64, t596: f64, t1029: f64, t1031: f64, t160: f64, t162: f64) -> (f64, f64, f64, f64) {
    let t3431 = (t3428 + t3429) * t158;
    let t3435 = t1746 * t3401;
    let t3438 = t596 * t3396;
    let t3441 = 6.0_f64 * t1029 * t1031 - 12.0_f64 * t160 * t3435 + 3.0_f64 * t160 * t3438 - t162 * t3431;
    (t3431, t3435, t3438, t3441)
}
