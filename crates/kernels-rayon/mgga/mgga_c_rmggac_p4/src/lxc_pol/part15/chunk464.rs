//! MGGA_C_RMGGAC lxc pol — lxc_pol part 15 (v4rho3sigma_6) CSE chunk 464/1110 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part15_v4rho3sigma_6_chunk464(t1173: f64, t615: f64, t495: f64, t1525: f64, t461: f64, t1510: f64, t4559: f64, t217: f64, t2184: f64, t1465: f64, t1470: f64, t1494: f64, t209: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t5554 = t1173 * t615;
    let t5555 = t5554 * t495;
    let t5558 = t461 * t1525;
    let t5571 = 0.25610252642437845428e0_f64 * t4559 * t1510;
    let t5582 = t2184 * t217;
    let t5583 = t1465 * t5582;
    let t5585 = 0.25610252642437845428e0_f64 * t5583 * t1470;
    let t5605 = t1494 * t209;
    (t5555, t5558, t5571, t5582, t5585, t5605)
}
