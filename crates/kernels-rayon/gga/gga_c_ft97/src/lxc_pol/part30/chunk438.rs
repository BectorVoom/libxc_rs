//! GGA_C_FT97 lxc pol — lxc_pol part 30 (v4rho2sigma2_11) CSE chunk 438/1184 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part30_v4rho2sigma2_11_chunk438(t27: f64, t7087: f64, t89: f64, t6316: f64, t6333: f64, t7065: f64, t7069: f64, t7073: f64, t7077: f64, t7081: f64, t7085: f64) -> (f64, f64) {
    let t7089 = t89 * t27 * t7087;
    let t7091 = t7065 / 12.0_f64 + t6316 + t7069 / 18.0_f64 + t7073 / 3.0_f64 - t7077 / 6.0_f64 + t6333 + t7081 / 9.0_f64 + 2.0_f64 / 3.0_f64 * t7085 - t7089 / 3.0_f64;
    (t7089, t7091)
}
