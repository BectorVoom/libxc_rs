//! GGA_C_GAPC lxc pol — lxc_pol part 32 (v4rho2sigma2_11) CSE chunk 17/1311 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gapc_lxc_pol_part32_v4rho2sigma2_11_chunk17(t36: f64, zeta_threshold: f64) -> (f64, f64, f64) {
    let cbrt2 = (M_CBRT2 as f64);
    let t34 = 1.0_f64 <= zeta_threshold;
    let t37 = piecewise3(t34, t36, 1.0_f64);
    let t40 = cbrt2;
    let t43 = 1.0_f64 / (2.0_f64 * t40 - 2.0_f64);
    (t37, t40, t43)
}
