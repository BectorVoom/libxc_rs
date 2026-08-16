//! MGGA_C_KCIS kxc pol — kxc_pol part 4 (v3rho3_1) CSE chunk 1336/1420 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_kxc_pol_part4_v3rho3_1_chunk1336(t1437: f64, t16073: f64, t1430: f64, t1451: f64, t16082: f64, t16060: f64, t542: f64, t16078: f64, t16069: f64, t1517: f64, t16055: f64, t16065: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t17210 = t1437 * t16073;
    let t17213 = t1430 * t16073;
    let t17216 = t1451 * t16073;
    let t17219 = t1430 * t16082;
    let t17222 = t542 * t16060;
    let t17225 = t1430 * t16078;
    let t17228 = t542 * t16069;
    let t17231 = t1517 * t16055;
    let t17234 = t542 * t16065;
    (t17210, t17213, t17216, t17219, t17222, t17225, t17228, t17231, t17234)
}
