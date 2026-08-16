//! GGA_C_ACGGAP lxc pol — lxc_pol part 11 (v4rho3sigma_3) CSE chunk 1090/1213 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part11_v4rho3sigma_3_chunk1090(t1181: f64, t33735: f64, t599: f64, t7413: f64, t1983: f64, t30127: f64, t7586: f64, t8791: f64, t21143: f64, t604: f64, t7493: f64, t22401: f64) -> (f64, f64, f64, f64) {
    let t34953 = t7413 * t1181 * t599 * t33735;
    let t34957 = t30127 * t7586 * t1983 * t8791;
    let t34958 = 0.28582678745379824648e-3_f64 * t34957;
    let t34961 = t7493 * t1181 * t604 * t21143;
    let t34962 = 0.31448092289604152068e-2_f64 * t34961;
    let t34965 = t7413 * t1181 * t604 * t22401;
    (t34953, t34958, t34962, t34965)
}
