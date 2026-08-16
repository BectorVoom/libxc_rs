//! MGGA_C_KCIS lxc pol — lxc_pol part 26 (v4rho3sigma_8) CSE chunk 1395/1397 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part26_v4rho3sigma_8_chunk1395(t101791: f64, t101807: f64, t101823: f64, t103953: f64, t29676: f64, t29679: f64, t8: f64, t93848: f64, t93849: f64, t93852: f64, t99792: f64, t99793: f64, t99794: f64, t99795: f64, t99796: f64) -> f64 {
    let t103957 = t29676 + t8 * (t101791 + t101807 + t101823 + t103953) + t93848 - t99792 - t99793 - t93849 - t99794 - t29679 + t99795 + t99796 + t93852;
    t103957
}
