//! MGGA_C_KCIS lxc pol — lxc_pol part 4 (v3rho3_1) CSE chunk 983/1420 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part4_v3rho3_1_chunk983(t10338: f64, t1106: f64, t3255: f64, t3285: f64, t3265: f64, t3296: f64, t346: f64, t9368: f64, t1018: f64, t127: f64, t368: f64, t245: f64, t313: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t10339 = t10338 * t1106;
    let t10341 = t3255 * t3285;
    let t10343 = t3255 * t3265;
    let t10351 = t3255 * t3296;
    let t10386 = t9368 * t346;
    let t10414 = t127 * t368 * t1018;
    let t10415 = t245 * t313;
    (t10339, t10341, t10343, t10351, t10386, t10414, t10415)
}
