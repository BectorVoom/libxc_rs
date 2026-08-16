//! GGA_C_ACGGAP lxc pol — lxc_pol part 5 (v4rho4_2) CSE chunk 603/1332 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part5_v4rho4_2_chunk603(t3770: f64, t390: f64, t1020: f64, t997: f64, t3055: f64, t383: f64) -> (f64, f64, f64) {
    let t3772 = 0.60023625365297631762e-2_f64 * t3770 * t390;
    let t3773 = t997 * t1020;
    let t3775 = t3055 * t383;
    (t3772, t3773, t3775)
}
