//! GGA_C_FT97 kxc pol — kxc_pol part 2 (v3rho3_1) CSE chunk 709/1007 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_kxc_pol_part2_v3rho3_1_chunk709(t11179: f64, t92: f64, t11059: f64, t1642: f64, t11013: f64, t3051: f64, t11034: f64, t11050: f64, t378: f64, t11003: f64, t10998: f64, t355: f64, t358: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t11180 = t92 * t11179;
    let t11182 = t1642 * t11059;
    let t11183 = t92 * t11182;
    let t11185 = t1642 * t11013;
    let t11186 = t3051 * t11185;
    let t11188 = t1642 * t11034;
    let t11189 = t92 * t11188;
    let t11191 = t378 * t11050;
    let t11192 = t92 * t11191;
    let t11194 = t378 * t11003;
    let t11195 = t3051 * t11194;
    let t11197 = t378 * t10998;
    let t11198 = t92 * t11197;
    let t11200 = t355 * t358;
    (t11180, t11183, t11186, t11189, t11192, t11195, t11198, t11200)
}
