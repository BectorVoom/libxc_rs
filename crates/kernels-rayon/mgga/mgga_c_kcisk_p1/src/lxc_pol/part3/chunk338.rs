//! MGGA_C_KCISK lxc pol — lxc_pol part 3 (v3rho3_0) CSE chunk 338/1063 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcisk_lxc_pol_part3_v3rho3_0_chunk338(t1634: f64, t1657: f64, t1638: f64, t1649: f64, t1654: f64, t1661: f64) -> f64 {
    let t1678 = 0.301925e0_f64 * t1634;
    let t1681 = 0.82785e-1_f64 * t1657;
    let t1683 = 0.258925e1_f64 * t1649 - t1678 - 0.301925e0_f64 * t1638 + 0.16504875e0_f64 * t1654 - t1681 - 0.82785e-1_f64 * t1661;
    t1683
}
