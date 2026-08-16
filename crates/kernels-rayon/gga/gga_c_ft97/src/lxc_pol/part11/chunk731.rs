//! GGA_C_FT97 lxc pol — lxc_pol part 11 (v4rho4_0) CSE chunk 731/1173 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part11_v4rho4_0_chunk731(t2493: f64, t9757: f64, t462: f64, t9897: f64, t9900: f64, t9903: f64, t9905: f64, t9907: f64, t9910: f64, t9913: f64, t9917: f64, t9922: f64, t9925: f64) -> (f64, f64) {
    let t9928 = t2493 * t9757;
    let t9930 = -2.0_f64 * t462 * t9897 - 2.0_f64 * t462 * t9900 - 2.0_f64 / 3.0_f64 * t9903 - 2.0_f64 / 3.0_f64 * t9905 - 4.0_f64 / 9.0_f64 * t9907 - 2.0_f64 * t462 * t9910 + 2.0_f64 * t462 * t9913 + 2.0_f64 / 3.0_f64 * t462 * t9917 + 4.0_f64 / 3.0_f64 * t462 * t9922 - 2.0_f64 / 3.0_f64 * t462 * t9925 + t462 * t9928;
    (t9928, t9930)
}
