//! GGA_C_FT97 lxc pol — lxc_pol part 30 (v4rho2sigma2_11) CSE chunk 728/1184 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part30_v4rho2sigma2_11_chunk728(t33319: f64, t684: f64, t9770: f64, t6118: f64, t33302: f64, t9942: f64, t1434: f64, t193: f64, t2506: f64, t33307: f64, t747: f64, t7484: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t33321 = t9770 * t33319 * t684;
    let t33322 = t6118 * t33321;
    let t33324 = t9942 * t33302;
    let t33326 = t1434 * t193 * t33324;
    let t33328 = t2506 * t33307;
    let t33330 = t1434 * t193 * t33328;
    let t33332 = t7484 * t747;
    (t33321, t33322, t33324, t33326, t33328, t33330, t33332)
}
