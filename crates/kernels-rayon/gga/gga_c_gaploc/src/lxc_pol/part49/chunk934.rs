//! GGA_C_GAPLOC lxc pol — lxc_pol part 49 (v4rhosigma3_14) CSE chunk 934/1217 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part49_v4rhosigma3_14_chunk934(t4130: f64, t41809: f64, t4781: f64, t590: f64, t493: f64, t1441: f64, t1339: f64, t41838: f64, t1537: f64, t18313: f64, t18372: f64, t41596: f64) -> (f64, f64, f64, f64, f64) {
    let t42047 = 0.15337170381568299871e1_f64 * t4781 * t4130 * t41809 * t590;
    let t42048 = t493 * t41809;
    let t42051 = 0.1022478025437886658e1_f64 * t1441 * t42048 * t590;
    let t42052 = t1339 * t41838;
    let t42054 = t1537 * t42052 * t590;
    let t42059 = 0.25561950635947166451e1_f64 * t1537 * t1339 * t41809 * t590;
    let t42064 = 0.61348681526273199482e1_f64 * t18372 * t18313 * t41596 * t590;
    (t42047, t42051, t42054, t42059, t42064)
}
