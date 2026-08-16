//! MGGA_C_KCISK lxc pol — lxc_pol part 3 (v3rho3_0) CSE chunk 878/1063 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcisk_lxc_pol_part3_v3rho3_0_chunk878(t1163: f64, t3619: f64, t3544: f64, t1417: f64, t3589: f64, t3595: f64, t3598: f64, t459: f64, t12970: f64, t457: f64, t3621: f64, t3521: f64, t3567: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t13175 = t1163 * t3619;
    let t13176 = t3544 * t13175;
    let t13179 = t1417 * t3589;
    let t13183 = t1417 * t3595;
    let t13185 = t3598 * t459;
    let t13186 = t13185 * t12970;
    let t13187 = t457 * t13186;
    let t13190 = t1417 * t3621;
    let t13192 = t3521 * t3567;
    (t13176, t13179, t13183, t13186, t13187, t13190, t13192)
}
