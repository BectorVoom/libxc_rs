//! GGA_C_ACGGAP lxc pol — lxc_pol part 12 (v4rho3sigma_4) CSE chunk 874/1250 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part12_v4rho3sigma_4_chunk874(t1973: f64, t7610: f64, t1985: f64, t30196: f64, t3668: f64, t587: f64, t381: f64, t390: f64, t151: f64) -> (f64, f64, f64, f64) {
    let t30240 = t7610 * t1973;
    let t30242 = t30196 * t1985;
    let t30244 = t587 * t3668;
    let t30246 = t381 * t30244 * t390;
    let t30248 = t151 * t30244;
    (t30240, t30242, t30246, t30248)
}
