//! MGGA_C_REVTPSS lxc pol — lxc_pol part 40 (v4rho3tau_3) CSE chunk 1432/1507 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part40_v4rho3tau_3_chunk1432(t13392: f64, t5302: f64, t1042: f64, t1252: f64, t1261: f64, t12956: f64, t17525: f64, t17529: f64, t17536: f64, t17541: f64, t17546: f64, t17547: f64, t17552: f64, t17556: f64, t3591: f64, t3606: f64, t3613: f64, t3711: f64, t5293: f64, t5299: f64) -> f64 {
    let t17557 = t5302 * t13392;
    let t17558 = t1042 * t17557;
    let t17561 = -0.11433071498151929859e-2_f64 * t5293 * t3591 - 0.22866142996303859718e-2_f64 * t17525 * t3606 + 0.11433071498151929859e-2_f64 * t17529 * t3613 + 0.28582678745379824648e-3_f64 * t12956 * t5299 + 0.28582678745379824648e-3_f64 * t3711 * t17536 + 0.14291339372689912324e-3_f64 * t3711 * t17541 + t17546 - 0.22866142996303859718e-2_f64 * t17547 * t1252 + 0.14291339372689912324e-2_f64 * t1261 * t17552 + t17556 + 0.23818898954483187207e-3_f64 * t1261 * t17558;
    t17561
}
