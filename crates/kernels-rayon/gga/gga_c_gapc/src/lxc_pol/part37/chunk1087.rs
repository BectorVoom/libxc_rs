//! GGA_C_GAPC lxc pol — lxc_pol part 37 (v4rho2sigma2_16) CSE chunk 1087/1445 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gapc_lxc_pol_part37_v4rho2sigma2_16_chunk1087(t2206: f64, t2394: f64, t2211: f64, t2254: f64, t102: f64, t327: f64, t959: f64, t285: f64, t6849: f64, t2762: f64, t328: f64, t332: f64) -> (f64, f64, f64, f64, f64) {
    let t18551 = t2394 * t2206;
    let t18553 = t2211 * t2254;
    let t18639 = t102 * t327 * t959;
    let t18679 = 1.0_f64 / t6849 / t285;
    let t18680 = 1.0_f64 / t2762 / t328 * t332 * t18679;
    (t18551, t18553, t18639, t18679, t18680)
}
