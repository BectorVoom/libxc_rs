//! MGGA_C_KCISK kxc pol — kxc_pol part 6 (v3rho3_3) CSE chunk 815/1086 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcisk_kxc_pol_part6_v3rho3_3_chunk815(t8126: f64, t960: f64, t8139: f64, t970: f64, t8142: f64, t8145: f64, t8148: f64, t965: f64, t443: f64, t7710: f64, t3859: f64, t1391: f64, t7706: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t25487 = t960 * t8126;
    let t25489 = t970 * t8139;
    let t25491 = t970 * t8142;
    let t25493 = t960 * t8145;
    let t25495 = t965 * t8148;
    let t25538 = t443 * t7710;
    let t25540 = t3859 * t7710;
    let t25542 = t1391 * t7706;
    (t25487, t25489, t25491, t25493, t25495, t25538, t25540, t25542)
}
