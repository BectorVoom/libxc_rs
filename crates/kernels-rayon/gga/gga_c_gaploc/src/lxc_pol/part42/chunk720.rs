//! GGA_C_GAPLOC lxc pol — lxc_pol part 42 (v4rhosigma3_7) CSE chunk 720/1012 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part42_v4rhosigma3_7_chunk720(t14364: f64, t808: f64, t568: f64, t836: f64, t314: f64, t313: f64, t739: f64, t531: f64, t2958: f64, t3720: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t14365 = t808 * t14364;
    let t14366 = t568 * t14365;
    let t14369 = t836 * t14364;
    let t14370 = t568 * t14369;
    let t14373 = t314 * t14364;
    let t14374 = t313 * t14373;
    let t14377 = t739 * t14364;
    let t14378 = t531 * t14377;
    let t14384 = t2958 * t3720;
    (t14365, t14366, t14369, t14370, t14373, t14374, t14377, t14378, t14384)
}
