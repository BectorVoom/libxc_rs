//! GGA_C_GAPLOC lxc pol — lxc_pol part 30 (v4rho2sigma2_13) CSE chunk 961/1436 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part30_v4rho2sigma2_13_chunk961(t10144: f64, t1457: f64, t1572: f64, t10123: f64, t8063: f64, t895: f64, t10156: f64, t188: f64, t10122: f64, t475: f64, t1445: f64, t10152: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t10477 = t1457 * t10144;
    let t10479 = 0.71500979903700853338e0_f64 * t1572 * t10477;
    let t10480 = t1457 * t10123;
    let t10484 = 0.23833659967900284446e0_f64 * t895 * t8063;
    let t10485 = t188 * t10156;
    let t10488 = t10122 * t475;
    let t10489 = t1445 * t10488;
    let t10492 = t1445 * t10152;
    (t10477, t10479, t10480, t10484, t10485, t10488, t10489, t10492)
}
