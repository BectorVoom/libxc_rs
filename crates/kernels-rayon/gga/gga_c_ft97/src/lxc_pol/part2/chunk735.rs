//! GGA_C_FT97 lxc pol — lxc_pol part 2 (v3rho3_1) CSE chunk 735/1007 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part2_v3rho3_1_chunk735(t1882: f64, t3235: f64, t8232: f64, t981: f64, t110: f64, t8326: f64, t10974: f64, t1780: f64, t488: f64, t1911: f64, t2983: f64, t1876: f64, t3238: f64, t452: f64) -> (f64, f64, f64, f64, f64) {
    let t11549 = 2.0_f64 / 9.0_f64 * t1882 * t3235;
    let t11550 = t8232 * t981;
    let t11552 = t8326 * t110;
    let t11553 = t11552 * t10974;
    let t11556 = t1780 * t488;
    let t11557 = t2983 * t1911;
    let t11558 = t11556 * t11557;
    let t11562 = t452 * t3238 * t1876;
    (t11549, t11550, t11553, t11558, t11562)
}
