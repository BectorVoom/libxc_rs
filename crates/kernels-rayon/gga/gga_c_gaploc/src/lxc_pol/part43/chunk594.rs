//! GGA_C_GAPLOC lxc pol — lxc_pol part 43 (v4rhosigma3_8) CSE chunk 594/1072 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part43_v4rhosigma3_8_chunk594(t10470: f64, t1537: f64, t10241: f64, t493: f64, t590: f64, t1441: f64, t10144: f64, t1457: f64, t1572: f64, t8063: f64, t895: f64, t3377: f64, t8155: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t10472 = 0.25561950635947166451e1_f64 * t1537 * t10470;
    let t10473 = t493 * t10241;
    let t10474 = t10473 * t590;
    let t10476 = 0.1022478025437886658e1_f64 * t1441 * t10474;
    let t10477 = t1457 * t10144;
    let t10479 = 0.71500979903700853338e0_f64 * t1572 * t10477;
    let t10484 = 0.23833659967900284446e0_f64 * t895 * t8063;
    let t10501 = 0.10725146985555128001e1_f64 * t8155 * t3377;
    (t10472, t10473, t10476, t10479, t10484, t10501)
}
