//! MGGA_C_KCISK kxc pol — kxc_pol part 3 (v3rho3_0) CSE chunk 626/1063 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcisk_kxc_pol_part3_v3rho3_0_chunk626(t2013: f64, t5480: f64, t2024: f64, t4419: f64, t782: f64, t4597: f64, t786: f64, t3290: f64, t5006: f64, t2020: f64, t695: f64, t1060: f64, t2023: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t5481 = t2013 * t5480;
    let t5483 = t4419 * t2024;
    let t5484 = t782 * t5483;
    let t5486 = t786 * t4597;
    let t5487 = t5486 * t3290;
    let t5488 = t5006 * t5487;
    let t5491 = t2020 * t695;
    let t5492 = t1060 * t2023;
    (t5481, t5483, t5484, t5486, t5487, t5488, t5491, t5492)
}
