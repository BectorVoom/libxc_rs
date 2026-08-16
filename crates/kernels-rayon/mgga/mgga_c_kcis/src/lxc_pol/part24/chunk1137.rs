//! MGGA_C_KCIS lxc pol — lxc_pol part 24 (v4rho3sigma_6) CSE chunk 1137/1322 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part24_v4rho3sigma_6_chunk1137(t7630: f64, t9268: f64, t2153: f64, t35635: f64, t9276: f64, t2539: f64, t9275: f64, t2770: f64, t7655: f64, t2161: f64, t9016: f64, t26439: f64, t710: f64, t86: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t91885 = 3.0_f64 * t9268 * t7630;
    let t91895 = 24.0_f64 * t35635 * t2153 * t9276;
    let t91901 = 18.0_f64 * t9275 * t7630 * t2539;
    let t91902 = t7655 * t2770;
    let t91905 = t2161 * t9016;
    let t91909 = t86 * t710 * t26439;
    (t91885, t91895, t91901, t91902, t91905, t91909)
}
