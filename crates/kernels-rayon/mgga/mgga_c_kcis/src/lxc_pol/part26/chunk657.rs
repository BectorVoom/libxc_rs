//! MGGA_C_KCIS lxc pol — lxc_pol part 26 (v4rho3sigma_8) CSE chunk 657/1397 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part26_v4rho3sigma_8_chunk657(t1600: f64, t7421: f64, t1601: f64, t6284: f64, t2104: f64) -> (f64, f64, f64, f64) {
    let t7422 = t1600 * t7421;
    let t7425 = t1601 * t6284;
    let t7426 = t1600 * t7425;
    let t7429 = t2104 * t2104;
    (t7422, t7425, t7426, t7429)
}
