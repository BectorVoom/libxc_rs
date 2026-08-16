//! GGA_C_FT97 lxc pol — lxc_pol part 30 (v4rho2sigma2_11) CSE chunk 1162/1184 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part30_v4rho2sigma2_11_chunk1162(t154111: f64, t154125: f64, t154143: f64, t154156: f64, t154173: f64, t154189: f64, t154204: f64, t154217: f64, t871: f64, t1882: f64, t36135: f64, t36261: f64) -> (f64, f64, f64) {
    let t154221 = t871 * (t154111 + t154125 + t154143 + t154156 + t154173 + t154189 + t154204 + t154217);
    let t154225 = t1882 * t36135;
    let t154235 = t1882 * t36261;
    (t154221, t154225, t154235)
}
