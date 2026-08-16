//! MGGA_C_TPSSLOC kxc pol — kxc_pol part 4 (v3rho3_2) CSE chunk 936/1228 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_kxc_pol_part4_v3rho3_2_chunk936(t2696: f64, t4166: f64, t849: f64, t13176: f64, t842: f64, t1516: f64, t9601: f64, t68: f64, t9971: f64, t226: f64, t4265: f64, t814: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t13360 = t4166 * t2696;
    let t13362 = 7.0_f64 / 576.0_f64 * t13360 * t849;
    let t13365 = t13176 * t842;
    let t13368 = t9601 * t1516;
    let t13396 = t68 * t9971;
    let t13397 = t226 * t13396;
    let t13433 = t814 * t4265;
    (t13360, t13362, t13365, t13368, t13397, t13433)
}
