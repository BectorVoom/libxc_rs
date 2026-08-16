//! MGGA_C_KCISK kxc pol — kxc_pol part 3 (v3rho3_0) CSE chunk 314/1063 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcisk_kxc_pol_part3_v3rho3_0_chunk314(t1492: f64, t470: f64, t486: f64, t1286: f64, t382: f64, t487: f64, t1404: f64, t467: f64, t492: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t1493 = t1492 * t470;
    let t1494 = t486 * t1493;
    let t1496 = t382 * t1286;
    let t1497 = t487 * t1496;
    let t1498 = t486 * t1497;
    let t1500 = t1404 * t467;
    let t1501 = t1500 * t492;
    (t1493, t1494, t1496, t1497, t1498, t1500, t1501)
}
