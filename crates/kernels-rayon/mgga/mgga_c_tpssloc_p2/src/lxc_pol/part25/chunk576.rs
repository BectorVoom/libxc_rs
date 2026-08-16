//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 25 (v4rho3sigma_1) CSE chunk 576/1226 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part25_v4rho3sigma_1_chunk576(t3499: f64, t3609: f64, t3507: f64, t491: f64, t1932: f64, t3508: f64, t1215: f64, t1235: f64, t1246: f64, t3493: f64, t1209: f64, t3032: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t3610 = t3499 * t3609;
    let t3611 = t491 * t3507;
    let t3612 = t1932 * t3508;
    let t3613 = t3611 * t3612;
    let t3616 = t1235 * t1215;
    let t3617 = t3616 * t1246;
    let t3620 = t491 * t3493;
    let t3621 = t3620 * t1246;
    let t3623 = t3032 * t1209;
    (t3610, t3611, t3612, t3613, t3617, t3620, t3621, t3623)
}
