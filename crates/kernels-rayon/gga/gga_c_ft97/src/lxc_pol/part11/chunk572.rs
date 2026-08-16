//! GGA_C_FT97 lxc pol — lxc_pol part 11 (v4rho4_0) CSE chunk 572/1173 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part11_v4rho4_0_chunk572(t378: f64, t7973: f64, t92: f64, t7945: f64, t7946: f64, t7948: f64, t7950: f64, t7952: f64, t7957: f64, t7961: f64, t7964: f64, t7968: f64, t7971: f64) -> (f64, f64, f64) {
    let t7974 = t378 * t7973;
    let t7975 = t92 * t7974;
    let t7977 = -t7945 - 4.0_f64 / 9.0_f64 * t7946 + 2.0_f64 / 9.0_f64 * t7948 - 2.0_f64 / 3.0_f64 * t7950 + t7952 / 3.0_f64 - 10.0_f64 / 27.0_f64 * t7957 + 4.0_f64 / 3.0_f64 * t7961 - 2.0_f64 / 3.0_f64 * t7964 - 2.0_f64 * t7968 + 2.0_f64 * t7971 - t7975 / 3.0_f64;
    (t7974, t7975, t7977)
}
