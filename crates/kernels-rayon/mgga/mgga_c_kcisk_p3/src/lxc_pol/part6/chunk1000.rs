//! MGGA_C_KCISK lxc pol — lxc_pol part 6 (v3rho3_3) CSE chunk 1000/1086 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcisk_lxc_pol_part6_v3rho3_3_chunk1000(t385: f64, t1284: f64, t30205: f64, t2147: f64, t2153: f64, t30476: f64, t30490: f64, t30494: f64, t340: f64, t379: f64, t382: f64, t6141: f64, t6142: f64, t8003: f64, t8011: f64, t8015: f64) -> (f64, f64) {
    let t386 = t385 < -0.66725e-1_f64;
    let t30498 = t1284 * t30205;
    let t30503 = piecewise3(t386, 0.0_f64, 10.0_f64 / 9.0_f64 * t340 * t30476 * t382 - 10.0_f64 / 9.0_f64 * t340 * t8003 * t2153 + 40.0_f64 / 27.0_f64 * t340 * t2147 * t8011 - 10.0_f64 / 9.0_f64 * t340 * t2147 * t8015 - 280.0_f64 / 243.0_f64 * t340 * t379 * t30490 + 40.0_f64 / 27.0_f64 * t6141 * t6142 * t30494 - 10.0_f64 / 27.0_f64 * t340 * t379 * t30498);
    (t30498, t30503)
}
