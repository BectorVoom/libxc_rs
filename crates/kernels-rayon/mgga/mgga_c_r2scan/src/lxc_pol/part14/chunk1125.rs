//! MGGA_C_R2SCAN lxc pol — lxc_pol part 14 (v4rho3sigma_4) CSE chunk 1125/1276 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_r2scan_lxc_pol_part14_v4rho3sigma_4_chunk1125(t10894: f64, t2630: f64, t10784: f64, t3613: f64, t5103: f64, t10844: f64, t11760: f64, t2201: f64, t3308: f64, t37965: f64, t7538: f64, t2214: f64, t3293: f64, t528: f64) -> (f64, f64, f64, f64, f64) {
    let t39601 = t10894 * t2630;
    let t39604 = t5103 * t3613 * t10784;
    let t39607 = t2201 * t11760 * t10844;
    let t39610 = t37965 * t3308 * t7538;
    let t39613 = t3293 * t2214 * t528;
    (t39601, t39604, t39607, t39610, t39613)
}
