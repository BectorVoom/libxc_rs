//! MGGA_C_KCISK kxc pol — kxc_pol part 3 (v3rho3_0) CSE chunk 534/1063 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcisk_kxc_pol_part3_v3rho3_0_chunk534(t1592: f64, t4419: f64, t535: f64, t3571: f64, t3573: f64, t3577: f64, t3581: f64, t3585: f64, t1524: f64, t1528: f64, t1527: f64, t512: f64) -> (f64, f64, f64, f64, f64) {
    let t4420 = t4419 * t1592;
    let t4421 = t535 * t4420;
    let t4423 = 0.22831111111111111111e-1_f64 * t3571;
    let t4428 = t4423 + 0.11415555555555555555e-1_f64 * t3573 - 0.11415555555555555555e-1_f64 * t3577 + 0.34246666666666666666e-1_f64 * t3581 - 0.17123333333333333333e-1_f64 * t3585;
    let t4431 = t1524 * t1528;
    let t4434 = t1527 * t512;
    (t4420, t4421, t4428, t4431, t4434)
}
