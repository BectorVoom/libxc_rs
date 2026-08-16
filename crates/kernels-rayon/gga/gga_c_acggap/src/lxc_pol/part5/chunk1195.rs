//! GGA_C_ACGGAP lxc pol — lxc_pol part 5 (v4rho4_2) CSE chunk 1195/1332 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part5_v4rho4_2_chunk1195(t13746: f64, t13748: f64, t13750: f64, t13754: f64, t13771: f64, t16233: f64, t16238: f64, t16241: f64, t16244: f64, t16249: f64, t16253: f64, t16255: f64, t16264: f64, t16274: f64) -> f64 {
    let t21735 = -0.1956e1_f64 * t16233 - 0.7335e0_f64 * t16238 + 0.489e0_f64 * t16241 + 0.2445e0_f64 * t16244 - 0.2445e1_f64 * t16249 + 0.9128e1_f64 * t16253 + 0.5868e1_f64 * t16255 - 0.1956e1_f64 * t16264 - 0.3912e1_f64 * t16274 - t13746 + 0.76066666666666666666e1_f64 * t13748 + 0.2282e1_f64 * t13750 - 0.1141e1_f64 * t13754 - 0.2445e1_f64 * t13771;
    t21735
}
