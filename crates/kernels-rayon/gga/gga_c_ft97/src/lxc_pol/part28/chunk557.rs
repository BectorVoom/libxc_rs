//! GGA_C_FT97 lxc pol — lxc_pol part 28 (v4rho2sigma2_6) CSE chunk 557/1189 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part28_v4rho2sigma2_6_chunk557(t1317: f64, t376: f64, t5684: f64, t1307: f64, t1570: f64, t1318: f64, t1637: f64, t5696: f64, t89: f64, t1316: f64, t458: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t23029 = t1317 * t376 * t5684;
    let t23031 = t1307 * t1570;
    let t23037 = t1317 * t1637 * t1318;
    let t23038 = 2.0_f64 / 9.0_f64 * t23037;
    let t23047 = t376 * t5696;
    let t23048 = t89 * t23047;
    let t23054 = t1316 * t458;
    (t23029, t23031, t23037, t23038, t23047, t23048, t23054)
}
