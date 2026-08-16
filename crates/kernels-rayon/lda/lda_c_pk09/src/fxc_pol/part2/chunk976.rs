//! LDA_C_PK09 fxc pol — fxc_pol part 2 (v2rho2_1) CSE chunk 976/1113 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pk09_fxc_pol_part2_v2rho2_1_chunk976(t1240: f64, t2640: f64, t1349: f64, t1337: f64, t623: f64, t5279: f64, t1364: f64, t2143: f64, t2649: f64, t310: f64, t2688: f64, t6175: f64) -> (f64, f64, f64, f64, f64) {
    let t10407 = t2640 * t1240;
    let t10408 = t1349 * t10407;
    let t10409 = t1337 * t10408;
    let t10411 = t2640 * t623;
    let t10412 = t5279 * t10411;
    let t10415 = t1364 * t2143;
    let t10416 = t1349 * t10415;
    let t10419 = t2649 * t1240;
    let t10420 = t310 * t10419;
    let t10421 = t1337 * t10420;
    let t10423 = t2688 * t6175;
    (t10409, t10412, t10416, t10421, t10423)
}
