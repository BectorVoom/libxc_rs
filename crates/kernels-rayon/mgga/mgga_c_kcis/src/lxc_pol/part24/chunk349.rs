//! MGGA_C_KCIS lxc pol — lxc_pol part 24 (v4rho3sigma_6) CSE chunk 349/1322 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part24_v4rho3sigma_6_chunk349(t1817: f64, t382: f64, t1798: f64, t1802: f64, t1806: f64, t1810: f64, t1814: f64) -> (f64, f64) {
    let t1818 = t382 * t1817;
    let t1820 = t1798 / 16.0_f64 - t1802 / 16.0_f64 + t1806 / 24.0_f64 - t1810 / 256.0_f64 + t1814 / 256.0_f64 - t1818 / 192.0_f64;
    (t1818, t1820)
}
