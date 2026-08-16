//! MGGA_C_KCISK lxc pol — lxc_pol part 6 (v3rho3_3) CSE chunk 1056/1086 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcisk_lxc_pol_part6_v3rho3_3_chunk1056(t31283: f64, t4203: f64, t14374: f64, t30489: f64, t498: f64, t493: f64, t2271: f64, t8279: f64, t30962: f64, t6322: f64, t4230: f64, t31133: f64, t470: f64) -> (f64, f64, f64, f64, f64) {
    let t31284 = t4203 * t31283;
    let t31286 = t14374 * t30489;
    let t31287 = t498 * t31286;
    let t31288 = t493 * t31287;
    let t31290 = t2271 * t8279;
    let t31292 = t6322 * t30962;
    let t31293 = t4230 * t31292;
    let t31295 = t470 * t31133;
    (t31284, t31288, t31290, t31293, t31295)
}
