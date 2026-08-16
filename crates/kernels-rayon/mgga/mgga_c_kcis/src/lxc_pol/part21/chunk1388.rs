//! MGGA_C_KCIS lxc pol — lxc_pol part 21 (v4rho3sigma_3) CSE chunk 1388/1389 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part21_v4rho3sigma_3_chunk1388(t26663: f64, t26666: f64, t26668: f64, t26670: f64, t91785: f64, t91786: f64, t95277: f64, t95278: f64, t95279: f64, t95280: f64, t95281: f64, t97609: f64) -> f64 {
    let tv4rho3sigma3 = t95277 + t26663 - t26666 - t91785 - t95278 - t95279 + t91786 + t26668 + t26670 - t95280 - t95281 + t97609;
    tv4rho3sigma3
}
