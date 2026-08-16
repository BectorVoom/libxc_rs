//! MGGA_C_KCISK kxc pol — kxc_pol part 6 (v3rho3_3) CSE chunk 363/1086 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcisk_kxc_pol_part6_v3rho3_3_chunk363(t529: f64, t2110: f64, t41: f64, t2153: f64, t382: f64, t525: f64, t526: f64, t79: f64, t534: f64) -> (f64, f64, f64) {
    let t530 = t529 < -0.66725e-1_f64;
    let t2308 = t2110 * t41;
    let t2316 = piecewise3(t530, 0.0_f64, 10.0_f64 / 9.0_f64 * t525 * t2308 * t382 - 10.0_f64 / 27.0_f64 * t525 * t526 * t2153);
    let t2317 = t79 * t2316;
    let t2318 = t2317 * t534;
    (t2308, t2317, t2318)
}
