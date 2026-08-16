//! GGA_C_GAPLOC lxc pol — lxc_pol part 39 (v4rhosigma3_4) CSE chunk 1123/1217 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part39_v4rhosigma3_4_chunk1123(t13870: f64, t795: f64, t313: f64, t2639: f64, t13857: f64, t4614: f64, t813: f64, t43682: f64, t47296: f64, t47299: f64, t47303: f64, t47306: f64, t47309: f64, t47315: f64, t47317: f64, t47321: f64, t47325: f64) -> (f64, f64) {
    let t47326 = t795 * t13870;
    let t47327 = t313 * t47326;
    let t47329 = 0.10725146985555128001e1_f64 * t47327 * t2639;
    let t47331 = t813 * t4614 * t13857;
    let t47333 = t43682 - 0.71500979903700853338e0_f64 * t47296 + 0.46011511144704899612e1_f64 * t47299 - t47303 + t47306 + t47309 + t47315 + t47317 + t47321 + t47325 - t47329 - 0.61348681526273199483e1_f64 * t47331;
    (t47326, t47333)
}
