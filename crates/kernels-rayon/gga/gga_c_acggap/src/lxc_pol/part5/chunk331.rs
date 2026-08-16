//! GGA_C_ACGGAP lxc pol — lxc_pol part 5 (v4rho4_2) CSE chunk 331/1332 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part5_v4rho4_2_chunk331(t336: f64, t337: f64, t922: f64, t1017: f64, t1083: f64, t150: f64, t394: f64) -> (f64, f64, f64) {
    let t1152 = t336 * t337 * t922;
    let t1156 = t336 * t1083 * t1017;
    let t1159 = t150 * t394;
    (t1152, t1156, t1159)
}
