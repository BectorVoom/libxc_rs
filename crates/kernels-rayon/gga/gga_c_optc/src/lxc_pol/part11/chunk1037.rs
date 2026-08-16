//! GGA_C_OPTC lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 1037/1451 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_optc_lxc_pol_part11_v4rho4_4_chunk1037(t24565: f64, t8125: f64, t2672: f64, t2748: f64, t24502: f64, t330: f64, t310: f64, t312: f64, t3648: f64, t307: f64, t23573: f64, t24391: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t24566 = t8125 * t24565;
    let t24568 = t2672 * t2672;
    let t24574 = t2748 * t24565;
    let t24583 = t330 * t24502;
    let t24599 = t310 * t3648 * t312;
    let t24601 = 0.18781521737197933637e-2_f64 * t307 * t24599;
    let t24619 = t24391 * t23573;
    (t24566, t24568, t24574, t24583, t24599, t24601, t24619)
}
