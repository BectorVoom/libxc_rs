//! GGA_C_ACGGAP lxc pol — lxc_pol part 16 (v4rho3sigma_8) CSE chunk 988/1223 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part16_v4rho3sigma_8_chunk988(t34957: f64, t1181: f64, t21143: f64, t604: f64, t7493: f64, t1992: f64, t5606: f64, t7585: f64, t7586: f64, t1432: f64, t30147: f64, t1494: f64, t7329: f64) -> (f64, f64, f64, f64, f64) {
    let t34958 = 0.28582678745379824648e-3_f64 * t34957;
    let t34961 = t7493 * t1181 * t604 * t21143;
    let t34962 = 0.31448092289604152068e-2_f64 * t34961;
    let t34990 = t7585 * t7586 * t1992 * t5606;
    let t34991 = 0.28582678745379824648e-3_f64 * t34990;
    let t35022 = t30147 * t7586 * t1992 * t1432;
    let t35039 = t7329 * t1494;
    (t34958, t34962, t34991, t35022, t35039)
}
