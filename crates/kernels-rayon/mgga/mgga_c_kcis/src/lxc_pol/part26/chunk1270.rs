//! MGGA_C_KCIS lxc pol — lxc_pol part 26 (v4rho3sigma_8) CSE chunk 1270/1397 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part26_v4rho3sigma_8_chunk1270(t1300: f64, t18433: f64, t1884: f64, t2233: f64, t2272: f64, t23379: f64, t29657: f64, t446: f64, t448: f64, t6260: f64, t6896: f64, t8014: f64, t91791: f64, t91793: f64, t91863: f64, t91866: f64, t91869: f64, t91872: f64, t91874: f64) -> f64 {
    let t101791 = -t91791 - t91793 - t91863 + t91866 - t91869 - t2233 * t1884 * t6260 / 8.0_f64 - t446 * t18433 * t2272 / 16.0_f64 - t2233 * t448 * t23379 / 16.0_f64 + t91872 - t91874 - t446 * t6896 * t8014 / 16.0_f64 - t446 * t1300 * t29657 / 16.0_f64;
    t101791
}
