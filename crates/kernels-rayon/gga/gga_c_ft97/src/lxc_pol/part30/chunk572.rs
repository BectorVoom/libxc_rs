//! GGA_C_FT97 lxc pol — lxc_pol part 30 (v4rho2sigma2_11) CSE chunk 572/1184 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part30_v4rho2sigma2_11_chunk572(t317: f64, t6260: f64, t1478: f64, t2399: f64, t6262: f64, t681: f64, t6266: f64, t1434: f64, t6891: f64, t668: f64, t6837: f64, t505: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t25465 = t6260 * t317;
    let t25485 = t2399 * t1478;
    let t25488 = t681 * t6262;
    let t25491 = t681 * t6266;
    let t27466 = t1434 * t681 * t6891;
    let t27468 = t6837 * t668;
    let t27469 = t27468 * t505;
    (t25465, t25485, t25488, t25491, t27466, t27469)
}
