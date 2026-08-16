//! GGA_C_FT97 kxc pol — kxc_pol part 3 (v3rho3_2) CSE chunk 514/1032 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_kxc_pol_part3_v3rho3_2_chunk514(t265: f64, t3821: f64, t729: f64, t1901: f64, t193: f64, t3877: f64, t3882: f64, t3888: f64, t3894: f64, t3899: f64, t3953: f64, t3958: f64, t3974: f64, t3979: f64, t3983: f64, t3986: f64, t3988: f64, t3991: f64, t3995: f64, t446: f64, t89: f64) -> (f64, f64) {
    let t3999 = t729 * t265 * t3821;
    let t4002 = t1901 * t3877 / 9.0_f64 + t1901 * t3882 / 9.0_f64 + 2.0_f64 / 9.0_f64 * t1901 * t3888 - 2.0_f64 / 27.0_f64 * t1901 * t3894 + t1901 * t3899 / 9.0_f64 + t89 * t193 * t3953 / 3.0_f64 - t3958 / 9.0_f64 - t446 * t3974 / 3.0_f64 - t446 * t3979 / 3.0_f64 - t446 * t3983 / 3.0_f64 + t3986 / 9.0_f64 + t3988 / 9.0_f64 - t446 * t3991 / 3.0_f64 - t446 * t3995 / 3.0_f64 - t446 * t3999 / 3.0_f64;
    (t3999, t4002)
}
