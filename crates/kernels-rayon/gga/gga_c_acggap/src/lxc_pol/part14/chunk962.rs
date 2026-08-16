//! GGA_C_ACGGAP lxc pol — lxc_pol part 14 (v4rho3sigma_6) CSE chunk 962/1223 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part14_v4rho3sigma_6_chunk962(t4680: f64, t7346: f64, t8896: f64, t7433: f64, t8962: f64, t30374: f64, t8657: f64, t30811: f64, t4904: f64, t2450: f64, t7431: f64, t8461: f64) -> (f64, f64, f64, f64, f64) {
    let t34130 = t7346 * t4680 * t8896;
    let t34131 = 0.21437009059034868486e-3_f64 * t34130;
    let t34132 = t7433 * t8962;
    let t34133 = 0.37737710747524982482e-2_f64 * t34132;
    let t34156 = t30374 * t8657;
    let t34158 = t30811 * t4904;
    let t34159 = 0.68598428988911579156e-2_f64 * t34158;
    let t34161 = t2450 * t7431 * t8461;
    (t34131, t34133, t34156, t34159, t34161)
}
