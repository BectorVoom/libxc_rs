//! MGGA_C_KCIS lxc pol — lxc_pol part 26 (v4rho3sigma_8) CSE chunk 613/1397 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part26_v4rho3sigma_8_chunk613(t1396: f64, t6912: f64, t1395: f64, t1394: f64, t2001: f64, t2011: f64) -> (f64, f64, f64, f64) {
    let t6913 = t1396 * t6912;
    let t6914 = t1395 * t6913;
    let t6915 = t1394 * t6914;
    let t6917 = t2001 * t2011;
    (t6913, t6914, t6915, t6917)
}
