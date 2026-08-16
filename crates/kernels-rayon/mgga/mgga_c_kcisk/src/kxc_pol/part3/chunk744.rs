//! MGGA_C_KCISK kxc pol — kxc_pol part 3 (v3rho3_0) CSE chunk 744/1063 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcisk_kxc_pol_part3_v3rho3_0_chunk744(t5094: f64, t696: f64, t1806: f64, t5102: f64, t143: f64, t4597: f64, t10441: f64, t682: f64, t1814: f64, t3290: f64, t1824: f64, t1810: f64, t3293: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t11491 = t696 * t5094;
    let t11493 = t1806 * t5102;
    let t11495 = t143 * t4597;
    let t11496 = t682 * t10441;
    let t11499 = t1814 * t3290;
    let t11500 = t11499 * t1824;
    let t11503 = t1810 * t3293;
    (t11491, t11493, t11495, t11496, t11500, t11503)
}
