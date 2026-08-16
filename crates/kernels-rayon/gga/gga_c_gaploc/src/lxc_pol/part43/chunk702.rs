//! GGA_C_GAPLOC lxc pol — lxc_pol part 43 (v4rhosigma3_8) CSE chunk 702/1072 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part43_v4rhosigma3_8_chunk702(t13086: f64, t471: f64, t3427: f64, t871: f64, t12555: f64, t12558: f64, t12566: f64, t12569: f64, t12580: f64, t12693: f64, t12697: f64, t12699: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t13087 = t13086 * t471;
    let t13088 = t3427 * t871;
    let t13091 = 9.0_f64 / 256.0_f64 * t12555;
    let t13092 = 9.0_f64 / 8192.0_f64 * t12558;
    let t13093 = 3.0_f64 / 8192.0_f64 * t12566;
    let t13094 = 3.0_f64 / 256.0_f64 * t12569;
    let t13095 = 2.0_f64 * t12580;
    let t13113 = 0.63904876589867916127e-1_f64 * t12693;
    let t13114 = 0.29792074959875355558e-1_f64 * t12697;
    let t13115 = 0.29792074959875355558e-1_f64 * t12699;
    (t13087, t13088, t13091, t13092, t13093, t13094, t13095, t13113, t13114, t13115)
}
