//! MGGA_C_PKZB lxc pol — lxc_pol part 9 (v4rho4_1) CSE chunk 1030/1336 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_pkzb_lxc_pol_part9_v4rho4_1_chunk1030(t2368: f64, t2370: f64, t8429: f64, t406: f64, t3207: f64, t8380: f64, t2387: f64, t394: f64, t3186: f64, t6456: f64, t8427: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t8436 = t2370 * t2368;
    let t8437 = t8429 * t8436;
    let t8438 = t406 * t8437;
    let t8441 = t8380 * t3207;
    let t8442 = t406 * t8441;
    let t8445 = t2387 * t394;
    let t8446 = t3186 * t8445;
    let t8447 = t406 * t8446;
    let t8450 = t6456 * t8427;
    (t8436, t8437, t8438, t8441, t8442, t8445, t8446, t8447, t8450)
}
