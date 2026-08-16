//! GGA_C_GAPLOC lxc pol — lxc_pol part 42 (v4rhosigma3_7) CSE chunk 827/1012 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part42_v4rhosigma3_7_chunk827(t44634: f64, t1063: f64, t36247: f64, t894: f64, t2268: f64, t426: f64, t44294: f64, t535: f64, t13273: f64, t484: f64, t11271: f64, t6763: f64) -> (f64, f64, f64, f64, f64) {
    let t44635 = 0.15808337019820083111e-2_f64 * t44634;
    let t44638 = 0.28455006635676149599e-1_f64 * t1063 * t894 * t36247;
    let t44642 = 0.28455006635676149599e-1_f64 * t2268 * t535 * t44294 * t426;
    let t44643 = t484 * t13273;
    let t44644 = 0.31616674039640166221e-2_f64 * t44643;
    let t44658 = 0.19918504644973304719e0_f64 * t2268 * t11271 * t6763;
    (t44635, t44638, t44642, t44644, t44658)
}
