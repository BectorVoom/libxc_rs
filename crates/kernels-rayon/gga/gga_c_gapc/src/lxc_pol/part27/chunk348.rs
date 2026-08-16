//! GGA_C_GAPC lxc pol — lxc_pol part 27 (v4rho2sigma2_6) CSE chunk 348/1310 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gapc_lxc_pol_part27_v4rho2sigma2_6_chunk348(t1524: f64, t493: f64, t124: f64, t4: f64, t495: f64, t128: f64, t511: f64, t8: f64, t134: f64, t122: f64, t186: f64, t21: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t1525 = t1524 * t493;
    let t1532 = t495 * t124 * t4;
    let t1535 = t1524 * t128;
    let t1539 = 1.0_f64 / t8 / t511;
    let t1540 = t1539 * t134;
    let t1543 = 1.0_f64 / t186 / t122;
    let t1545 = t1543 * t124 * t21;
    (t1525, t1532, t1535, t1539, t1540, t1545)
}
