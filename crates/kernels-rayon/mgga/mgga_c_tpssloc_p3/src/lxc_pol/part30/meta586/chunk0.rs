//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 30 (v4rho3sigma_6) CSE chunk 1965/2341 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1965(t16752: f64, t252: f64, t5527: f64, t828: f64, t5611: f64, t5584: f64, t9975: f64, t852: f64, t17100: f64, t225: f64, t17087: f64, t17060: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t58262 = t252 * t16752;
    let t58557 = t5527 * t828;
    let t58569 = t5611 * t828;
    let t58688 = t5584 * t828;
    let t58853 = t5584 * t9975;
    let t59331 = t852 * t5611;
    let t59466 = t17100 * t225;
    let t59498 = t17087 * t225;
    let t59503 = t17060 * t225;
    (t58262, t58557, t58569, t58688, t58853, t59331, t59466, t59498, t59503)
}
