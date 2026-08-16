//! GGA_C_GAPLOC lxc pol — lxc_pol part 33 (v4rho2sigma2_16) CSE chunk 1315/1464 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part33_v4rho2sigma2_16_chunk1315(t20827: f64, t34459: f64, t6717: f64, t10314: f64, t20441: f64, t6716: f64, t26984: f64, t7026: f64, t10532: f64, t10533: f64, t34246: f64, t1397: f64, t8410: f64) -> (f64, f64, f64, f64, f64) {
    let t34462 = 0.13803453343411469884e2_f64 * t20827 * t6717 * t34459;
    let t34465 = 0.18404604457881959845e2_f64 * t6716 * t20441 * t10314;
    let t34466 = t26984 * t7026;
    let t34467 = 0.89376224879626066674e-1_f64 * t34466;
    let t34470 = 0.27606906686822939767e2_f64 * t10532 * t10533 * t34246;
    let t34471 = t1397 * t8410;
    (t34462, t34465, t34467, t34470, t34471)
}
