//! MGGA_C_REVTPSS lxc pol — lxc_pol part 56 (v4rho2sigma2_11) CSE chunk 1121/1203 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part56_v4rho2sigma2_11_chunk1121(t119757: f64, t31846: f64, t4461: f64, t119752: f64, t4446: f64, t119751: f64, t33714: f64, t837: f64, t119783: f64, t4365: f64, t1579: f64, t775: f64) -> (f64, f64, f64, f64, f64) {
    let t126065 = t31846 * t119757 * t4461;
    let t126068 = t31846 * t119752 * t4446;
    let t126072 = t119751 * t119752 * t33714 * t837;
    let t126076 = t119751 * t119752 * t4365 * t119783;
    let t126078 = t1579 * t775;
    (t126065, t126068, t126072, t126076, t126078)
}
