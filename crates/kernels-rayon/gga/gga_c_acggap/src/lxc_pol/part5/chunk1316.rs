//! GGA_C_ACGGAP lxc pol — lxc_pol part 5 (v4rho4_2) CSE chunk 1316/1332 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part5_v4rho4_2_chunk1316(t11739: f64, t11743: f64, t11747: f64, t14974: f64, t1680: f64, t19973: f64, t19974: f64, t19975: f64, t19977: f64, t19978: f64, t19979: f64, t19980: f64, t694: f64) -> f64 {
    let t24564 = -6.0_f64 * t14974 * t1680 * t694 + t11739 - t11743 + t11747 + t19973 + t19974 + t19975 - t19977 - t19978 + t19979 - t19980;
    t24564
}
