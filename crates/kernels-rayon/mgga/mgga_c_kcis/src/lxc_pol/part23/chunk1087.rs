//! MGGA_C_KCIS lxc pol — lxc_pol part 23 (v4rho3sigma_5) CSE chunk 1087/1323 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part23_v4rho3sigma_5_chunk1087(t28356: f64, t7924: f64, t1394: f64, t1489: f64, t2046: f64, t27387: f64, t1464: f64, t491: f64, t5742: f64, t990: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t28357 = t28356 * t7924;
    let t28358 = t1394 * t28357;
    let t28360 = t2046 * t1489;
    let t28361 = t27387 * t28360;
    let t28362 = t1464 * t28361;
    let t28369 = t5742 * t491 * t990;
    (t28357, t28358, t28360, t28361, t28362, t28369)
}
