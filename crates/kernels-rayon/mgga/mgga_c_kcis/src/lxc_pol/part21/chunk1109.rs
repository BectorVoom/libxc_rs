//! MGGA_C_KCIS lxc pol — lxc_pol part 21 (v4rho3sigma_3) CSE chunk 1109/1389 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part21_v4rho3sigma_3_chunk1109(t1262: f64, t1267: f64, t26996: f64, t5329: f64, t2845: f64, t7789: f64, t3507: f64, t3500: f64, t7790: f64, t7788: f64, t2829: f64, t1252: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t26997 = t1262 * t1267;
    let t26998 = t26996 * t26997;
    let t26999 = t5329 * t26998;
    let t27002 = t7789 * t2845;
    let t27003 = t3507 * t27002;
    let t27006 = t3500 * t7790;
    let t27007 = t7788 * t27006;
    let t27009 = t7789 * t2829;
    let t27010 = t1252 * t27009;
    (t26997, t26998, t26999, t27002, t27003, t27006, t27007, t27009, t27010)
}
