//! GGA_C_FT97 lxc pol — lxc_pol part 30 (v4rho2sigma2_11) CSE chunk 894/1184 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part30_v4rho2sigma2_11_chunk894(t6353: f64, t7105: f64, t840: f64, t296: f64, t36005: f64, t1255: f64, t2862: f64, t7584: f64, t34191: f64, t34193: f64, t34195: f64, t34241: f64, t36242: f64, t36246: f64, t36250: f64, t36253: f64, t36257: f64, t446: f64) -> (f64, f64, f64, f64) {
    let t36261 = t840 * t6353 * t7105;
    let t36264 = t296 * t36005;
    let t36268 = t2862 * t1255 * t7584;
    let t36271 = -2.0_f64 / 3.0_f64 * t446 * t36242 + 4.0_f64 / 3.0_f64 * t446 * t36246 + 2.0_f64 / 3.0_f64 * t446 * t36250 + t34191 + t34193 - t34195 - t446 * t36253 / 3.0_f64 - t446 * t36257 / 9.0_f64 + 2.0_f64 / 3.0_f64 * t446 * t36261 - 2.0_f64 / 3.0_f64 * t446 * t36264 + 2.0_f64 / 3.0_f64 * t446 * t36268 - t34241;
    (t36261, t36264, t36268, t36271)
}
