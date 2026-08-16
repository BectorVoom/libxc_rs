//! GGA_C_GAPLOC lxc pol — lxc_pol part 30 (v4rho2sigma2_13) CSE chunk 297/1436 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part30_v4rho2sigma2_13_chunk297(t1230: f64, t1255: f64, t1257: f64, t157: f64, t470: f64, t471: f64, t64: f64, t90: f64) -> f64 {
    let t1265 = t1257 * t471 - 4.0_f64 / 3.0_f64 * t470 * t64 + 7.0_f64 / 96.0_f64 * t1230 - 7.0_f64 / 288.0_f64 * t1255 + 4.0_f64 / 3.0_f64 * t157 * t90;
    t1265
}
