//! GGA_C_FT97 lxc pol — lxc_pol part 15 (v4rho4_4) CSE chunk 942/1222 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part15_v4rho4_4_chunk942(t20435: f64, t8392: f64, t1882: f64, t20401: f64, t20226: f64, t20439: f64, t20405: f64, t20256: f64, t20200: f64, t103: f64, t20113: f64, t20260: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t75050 = t8392 * t20435;
    let t75071 = t1882 * t20401;
    let t75115 = t8392 * t20226;
    let t75117 = t8392 * t20439;
    let t75119 = t1882 * t20405;
    let t75136 = t1882 * t20256;
    let t75138 = t1882 * t20200;
    let t75188 = t103 * t20113;
    let t75227 = t1882 * t20260;
    (t75050, t75071, t75115, t75117, t75119, t75136, t75138, t75188, t75227)
}
