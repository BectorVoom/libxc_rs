//! MGGA_C_KCIS lxc pol — lxc_pol part 5 (v3rho3_2) CSE chunk 883/1419 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part5_v3rho3_2_chunk883(t1533: f64, t7335: f64, t4261: f64, t6917: f64, t4260: f64, t143: f64, t7028: f64, t4219: f64, t4220: f64, t6281: f64, t1517: f64, t1650: f64, t5987: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t7336 = t1533 * t7335;
    let t7338 = t4261 * t6917;
    let t7339 = t4260 * t7338;
    let t7341 = t7028 * t143;
    let t7361 = t4219 * t4220 * t6281;
    let t7365 = t1517 * t5987 * t1650;
    (t7336, t7338, t7339, t7341, t7361, t7365)
}
