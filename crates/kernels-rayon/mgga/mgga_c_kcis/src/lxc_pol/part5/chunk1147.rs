//! MGGA_C_KCIS lxc pol — lxc_pol part 5 (v3rho3_2) CSE chunk 1147/1419 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part5_v3rho3_2_chunk1147(t19186: f64, t19214: f64, t19252: f64, t19292: f64, t1009: f64, t4824: f64, t5026: f64, t1092: f64, t4773: f64, t3178: f64, t6614: f64, t2855: f64, t6486: f64) -> (f64, f64, f64, f64, f64) {
    let t19294 = t19186 + t19214 + t19252 + t19292;
    let t19295 = t19294 * t1009;
    let t19300 = t5026 * t4824;
    let t19301 = t1092 * t19300;
    let t19303 = t5026 * t4773;
    let t19304 = t1092 * t19303;
    let t19306 = t3178 * t6614;
    let t19307 = t1092 * t19306;
    let t19309 = t2855 * t6486;
    (t19295, t19301, t19304, t19307, t19309)
}
