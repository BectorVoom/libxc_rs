//! MGGA_C_KCISK lxc pol — lxc_pol part 6 (v3rho3_3) CSE chunk 142/1086 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcisk_lxc_pol_part6_v3rho3_3_chunk142(t303: f64, t306: f64, t309: f64, t315: f64, t240: f64, t323: f64, t335: f64, t507: f64) -> (f64, f64, f64, f64) {
    let t512 = 0.705945e1_f64 * t306 + 0.1549425e1_f64 * t303 + 0.420775e0_f64 * t309 + 0.1562925e0_f64 * t315;
    let t515 = 1.0_f64 + 0.32164683177870697974e2_f64 / t512;
    let t516 = f64::ln(t515);
    let t524 = -t323 + t240 * (-0.3109e-1_f64 * t507 * t516 + t323 - 0.19751789702565206229e-1_f64 * t335) + 0.19751789702565206229e-1_f64 * t240 * t335;
    (t512, t515, t516, t524)
}
