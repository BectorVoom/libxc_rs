//! GGA_C_FT97 lxc pol — lxc_pol part 28 (v4rho2sigma2_6) CSE chunk 360/1189 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part28_v4rho2sigma2_6_chunk360(t108: f64, t5618: f64, t28: f64, t1308: f64, t497: f64, t1314: f64, t1882: f64, t103: f64, t1307: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t5619 = t5618 * t108;
    let t5620 = t28 * t5619;
    let t5623 = t1308 * t497;
    let t5624 = t28 * t5623;
    let t5629 = t1882 * t1314 / 9.0_f64;
    let t5630 = t103 * t1307;
    (t5619, t5620, t5623, t5624, t5629, t5630)
}
