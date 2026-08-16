//! GGA_C_GAPLOC lxc pol — lxc_pol part 52 (v4rhosigma3_17) CSE chunk 717/1013 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part52_v4rhosigma3_17_chunk717(t14271: f64, t1457: f64, t12961: f64, t12988: f64, t13442: f64, t13444: f64, t13463: f64, t13466: f64, t13469: f64, t13473: f64, t13477: f64, t13478: f64, t13480: f64, t1572: f64) -> (f64, f64) {
    let t14340 = t1457 * t14271;
    let t14346 = -t13442 - t13444 + 0.38342925953920749676e1_f64 * t12961 - t13463 + 0.14300195980740170668e1_f64 * t1572 * t14340 + 0.63904876589867916127e-1_f64 * t12988 - 0.38342925953920749676e0_f64 * t13466 - 0.57514388930881124514e0_f64 * t13469 + t13473 + t13477 + t13478 + t13480;
    (t14340, t14346)
}
