//! GGA_C_FT97 lxc pol — lxc_pol part 30 (v4rho2sigma2_11) CSE chunk 947/1184 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part30_v4rho2sigma2_11_chunk947(t24543: f64, t33321: f64, t24237: f64, t33496: f64, t1403: f64, t2399: f64, t7490: f64, t33247: f64, t681: f64, t263: f64, t33452: f64, t33537: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t141384 = t24543 * t33321;
    let t141406 = t24237 * t33496;
    let t141410 = 4.0_f64 / 27.0_f64 * t1403 * t2399 * t7490;
    let t141420 = t1403 * t681 * t33247;
    let t141422 = t33452 * t263;
    let t141431 = t24237 * t33537;
    (t141384, t141406, t141410, t141420, t141422, t141431)
}
