//! MGGA_C_KCISK kxc pol — kxc_pol part 3 (v3rho3_0) CSE chunk 969/1063 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcisk_kxc_pol_part3_v3rho3_0_chunk969(t1445: f64, t1486: f64, t4205: f64, t13320: f64, t4204: f64, t4203: f64, t4223: f64, t4226: f64, t13949: f64, t4231: f64, t4230: f64, t1492: f64, t4210: f64) -> (f64, f64, f64, f64, f64) {
    let t14304 = t1486 * t1445;
    let t14305 = t14304 * t4205;
    let t14307 = t4204 * t13320;
    let t14308 = t4203 * t14307;
    let t14310 = t4223 * t4226;
    let t14312 = t4231 * t13949;
    let t14313 = t4230 * t14312;
    let t14315 = t1492 * t4210;
    (t14305, t14308, t14310, t14313, t14315)
}
