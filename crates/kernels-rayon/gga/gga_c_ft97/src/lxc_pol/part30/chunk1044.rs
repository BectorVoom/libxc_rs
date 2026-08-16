//! GGA_C_FT97 lxc pol — lxc_pol part 30 (v4rho2sigma2_11) CSE chunk 1044/1184 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part30_v4rho2sigma2_11_chunk1044(t3821: f64, t7484: f64, t1434: f64, t193: f64, t2506: f64, t35516: f64, t6109: f64, t743: f64, t747: f64, t10157: f64, t24437: f64, t27796: f64, t33319: f64) -> (f64, f64, f64, f64) {
    let t150928 = t7484 * t3821;
    let t150931 = t1434 * t193 * t2506 * t150928;
    let t150935 = t6109 * t193 * t743 * t35516 * t747;
    let t150939 = t24437 * t10157 * t33319 * t27796;
    (t150928, t150931, t150935, t150939)
}
