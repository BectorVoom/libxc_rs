//! GGA_C_GAPC lxc pol — lxc_pol part 33 (v4rho2sigma2_12) CSE chunk 1158/1306 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gapc_lxc_pol_part33_v4rho2sigma2_12_chunk1158(t11577: f64, t11580: f64, t561: f64, t21643: f64, t26561: f64, t1743: f64, t26597: f64, t21801: f64, t5395: f64, t5743: f64, t5722: f64, t1030: f64, t33311: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t34359 = t561 * t11577 * t11580;
    let t34361 = t26561 * t21643;
    let t34363 = t1743 * t26597;
    let t34364 = t34363 * t21643;
    let t34366 = t5395 * t21801;
    let t34367 = t34366 * t5743;
    let t34370 = t1743 * t21801 * t5722;
    let t34372 = t1030 * t33311;
    (t34359, t34361, t34363, t34364, t34366, t34367, t34370, t34372)
}
