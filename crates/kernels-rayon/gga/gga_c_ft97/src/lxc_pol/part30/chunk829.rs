//! GGA_C_FT97 lxc pol — lxc_pol part 30 (v4rho2sigma2_11) CSE chunk 829/1184 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part30_v4rho2sigma2_11_chunk829(t2506: f64, t35353: f64, t1434: f64, t193: f64, t33414: f64, t6832: f64, t1096: f64, t33427: f64, t204: f64, t6817: f64) -> (f64, f64, f64, f64, f64) {
    let t35354 = t2506 * t35353;
    let t35356 = t1434 * t193 * t35354;
    let t35358 = t33414 * t6832;
    let t35361 = t33427 * t1096;
    let t35367 = t204 * t6817;
    (t35354, t35356, t35358, t35361, t35367)
}
