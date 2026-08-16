//! GGA_C_FT97 lxc pol — lxc_pol part 11 (v4rho4_0) CSE chunk 643/1173 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part11_v4rho4_0_chunk643(t1595: f64, t422: f64, t120: f64, t358: f64, t363: f64, t528: f64, t7899: f64, t72: f64, t123: f64, t532: f64, t7911: f64, t126: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t8964 = t422 * t1595;
    let t8965 = t120 * t358;
    let t8966 = t8965 * t363;
    let t8967 = t8964 * t8966;
    let t8971 = t7899 * t528 * t120;
    let t8972 = t72 * t8971;
    let t8977 = t123 / t532 / t7911;
    let t8978 = t7899 * t126;
    (t8964, t8965, t8966, t8967, t8972, t8977, t8978)
}
