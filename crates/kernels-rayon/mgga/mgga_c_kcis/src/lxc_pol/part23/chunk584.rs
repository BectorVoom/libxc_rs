//! MGGA_C_KCIS lxc pol — lxc_pol part 23 (v4rho3sigma_5) CSE chunk 584/1323 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part23_v4rho3sigma_5_chunk584(t4171: f64, t5648: f64, t4170: f64, t4160: f64, t1444: f64, t556: f64, t1650: f64, t833: f64) -> (f64, f64, f64, f64, f64) {
    let t5649 = t4171 * t5648;
    let t5650 = t4170 * t5649;
    let t5651 = t4160 * t5650;
    let t5653 = t556 * t1444;
    let t5654 = t1650 * t833;
    (t5649, t5650, t5651, t5653, t5654)
}
