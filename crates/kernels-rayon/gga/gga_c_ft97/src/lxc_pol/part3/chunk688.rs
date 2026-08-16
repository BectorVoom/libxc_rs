//! GGA_C_FT97 lxc pol — lxc_pol part 3 (v3rho3_2) CSE chunk 688/1032 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part3_v3rho3_2_chunk688(t1882: f64, t3240: f64, t3235: f64, t8232: f64, t981: f64, t110: f64, t8326: f64, t1780: f64, t488: f64, t3172: f64, t376: f64, t89: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t11537 = 2.0_f64 / 9.0_f64 * t1882 * t3240;
    let t11549 = 2.0_f64 / 9.0_f64 * t1882 * t3235;
    let t11550 = t8232 * t981;
    let t11552 = t8326 * t110;
    let t11556 = t1780 * t488;
    let t11567 = 2.0_f64 / 9.0_f64 * t89 * t376 * t3172;
    (t11537, t11549, t11550, t11552, t11556, t11567)
}
