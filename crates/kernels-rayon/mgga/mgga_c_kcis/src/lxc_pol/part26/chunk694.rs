//! MGGA_C_KCIS lxc pol — lxc_pol part 26 (v4rho3sigma_8) CSE chunk 694/1397 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part26_v4rho3sigma_8_chunk694(t2237: f64, t7904: f64, t1364: f64, t491: f64, t990: f64) -> (f64, f64, f64) {
    let t7906 = 0.23168402777777777778e-3_f64 * t2237 * t7904;
    let t7907 = t1364 * t491;
    let t7908 = t7907 * t990;
    (t7906, t7907, t7908)
}
