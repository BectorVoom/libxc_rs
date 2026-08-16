//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 20 (v4rho4_1) CSE chunk 2294/2712 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2294(t13396: f64, t808: f64, t1509: f64, t2710: f64, t4233: f64, t852: f64, t13170: f64, t252: f64, t10084: f64, t10101: f64, t13176: f64, t13263: f64, t13380: f64, t13384: f64, t13397: f64, t13401: f64, t13404: f64, t13453: f64, t2684: f64, t2733: f64, t4166: f64, t4182: f64, t4281: f64, t4282: f64, t4291: f64, t829: f64, t9661: f64) -> (f64, f64, f64, f64, f64) {
    let t47419 = t808 * t13396;
    let t47425 = t2710 * t1509;
    let t47439 = t852 * t4233;
    let t47448 = t252 * t13170;
    let t47452 = -18.0_f64 * t13263 * t13380 * t13397 - 3.0_f64 * t13384 * t2684 * t4291 + 6.0_f64 * t4182 * t4281 * t47425 + 12.0_f64 * t4182 * t4281 * t47439 - t4282 * t4291 * t9661 - 3.0_f64 * t4291 * t47448 * t829 + 6.0_f64 * t10084 * t4166 - t10101 * t4166 - 6.0_f64 * t13176 * t2733 + 18.0_f64 * t13401 * t13453 + 6.0_f64 * t13404 * t13453;
    (t47419, t47425, t47439, t47448, t47452)
}
