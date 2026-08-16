//! GGA_C_FT97 lxc pol — lxc_pol part 11 (v4rho4_0) CSE chunk 1138/1173 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part11_v4rho4_0_chunk1138(t2: f64, t43917: f64, t10603: f64, t2766: f64, t2771: f64, t41482: f64, t4206: f64, t42083: f64, t42154: f64, t43351: f64, t43355: f64, t43367: f64, t43371: f64, t43382: f64, t43888: f64, t43890: f64, t43904: f64, t43906: f64, t43908: f64, t43910: f64, t43913: f64, t462: f64) -> f64 {
    let t43918 = t43917 * t2;
    let t43922 = -8.0_f64 * t462 * t2766 * t42083 - 2.0_f64 / 3.0_f64 * t462 * t2766 * t42154 + 4.0_f64 / 9.0_f64 * t43888 - 4.0_f64 / 3.0_f64 * t43890 - 4.0_f64 * t462 * t10603 * t43367 - 4.0_f64 * t462 * t2771 * t43355 + 8.0_f64 * t462 * t2771 * t43371 - 12.0_f64 * t462 * t4206 * t41482 - 8.0_f64 / 9.0_f64 * t43904 + 16.0_f64 / 9.0_f64 * t43906 - 8.0_f64 / 3.0_f64 * t43908 + 8.0_f64 / 3.0_f64 * t43910 + 8.0_f64 * t462 * t43913 * t43382 - 8.0_f64 / 3.0_f64 * t462 * t43918 * t43351;
    t43922
}
