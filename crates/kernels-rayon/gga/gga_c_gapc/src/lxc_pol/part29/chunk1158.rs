//! GGA_C_GAPC lxc pol — lxc_pol part 29 (v4rho2sigma2_8) CSE chunk 1158/1311 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gapc_lxc_pol_part29_v4rho2sigma2_8_chunk1158(t2964: f64, t9370: f64, t3179: f64, t1616: f64, t1615: f64, t3655: f64, t1617: f64, t11302: f64, t19844: f64, t5974: f64, t1743: f64, t33148: f64) -> (f64, f64, f64, f64, f64) {
    let t34303 = 2.0_f64 * t2964 * t9370;
    let t34306 = t3179 * t3179;
    let t34308 = 4.0_f64 * t1616 * t34306;
    let t34311 = t3655 * t1615;
    let t34313 = 2.0_f64 * t34311 * t1617;
    let t34315 = t19844 * t11302 * t5974;
    let t34317 = t1743 * t33148;
    (t34303, t34308, t34313, t34315, t34317)
}
