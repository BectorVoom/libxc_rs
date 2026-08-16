//! GGA_C_FT97 kxc pol — kxc_pol part 3 (v3rho3_2) CSE chunk 927/1032 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_kxc_pol_part3_v3rho3_2_chunk927(t17757: f64, t2493: f64, t17736: f64, t17776: f64, t9896: f64, t17740: f64, t17744: f64, t3917: f64, t17780: f64, t17722: f64, t18303: f64, t18305: f64, t18308: f64, t18312: f64, t18314: f64, t18316: f64, t18318: f64, t18321: f64, t18324: f64, t18327: f64, t18330: f64, t18333: f64, t18336: f64, t3139: f64, t462: f64, t92: f64) -> f64 {
    let t18339 = t2493 * t17757;
    let t18342 = t2493 * t17736;
    let t18345 = t9896 * t17776;
    let t18348 = t2493 * t17740;
    let t18351 = t3917 * t17744;
    let t18354 = t3917 * t17780;
    let t18357 = t2493 * t17722;
    let t18360 = 2.0_f64 / 27.0_f64 * t18303 - 2.0_f64 / 9.0_f64 * t18305 - t462 * t18308 / 3.0_f64 - t92 * t18312 + t18314 / 3.0_f64 - 2.0_f64 / 3.0_f64 * t18316 + 2.0_f64 / 3.0_f64 * t462 * t18318 - 2.0_f64 / 9.0_f64 * t462 * t18321 + t462 * t18324 / 3.0_f64 + 2.0_f64 / 9.0_f64 * t462 * t18327 + 4.0_f64 / 3.0_f64 * t462 * t18330 - 10.0_f64 / 27.0_f64 * t462 * t18333 + 8.0_f64 / 9.0_f64 * t3139 * t18336 + 2.0_f64 / 3.0_f64 * t462 * t18339 - 4.0_f64 / 3.0_f64 * t3139 * t18342 - 2.0_f64 / 3.0_f64 * t462 * t18345 - 2.0_f64 / 3.0_f64 * t462 * t18348 - 2.0_f64 * t462 * t18351 - 8.0_f64 / 3.0_f64 * t3139 * t18354 + t462 * t18357 / 3.0_f64;
    t18360
}
