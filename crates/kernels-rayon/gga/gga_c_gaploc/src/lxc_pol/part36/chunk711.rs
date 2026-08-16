//! GGA_C_GAPLOC lxc pol — lxc_pol part 36 (v4rhosigma3_1) CSE chunk 711/1029 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part36_v4rhosigma3_1_chunk711(t10924: f64, t787: f64, t9824: f64, t12555: f64, t12558: f64, t12561: f64, t12564: f64, t12566: f64, t12569: f64, t471: f64, t3427: f64, t871: f64) -> (f64, f64, f64, f64, f64) {
    let t13077 = t787 * t10924;
    let t13078 = t13077 * t9824;
    let t13079 = 0.29792074959875355558e-1_f64 * t13078;
    let t13086 = -3.0_f64 / 256.0_f64 * t12555 - 27.0_f64 / 8192.0_f64 * t12558 + 27.0_f64 / 524288.0_f64 * t12561 - 9.0_f64 / 524288.0_f64 * t12564 + 9.0_f64 / 8192.0_f64 * t12566 + t12569 / 256.0_f64;
    let t13087 = t13086 * t471;
    let t13088 = t3427 * t871;
    (t13077, t13079, t13086, t13087, t13088)
}
