//! GGA_C_GAPLOC lxc pol — lxc_pol part 39 (v4rhosigma3_4) CSE chunk 648/1217 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part39_v4rhosigma3_4_chunk648(t10619: f64, t10611: f64, t10612: f64, t10617: f64, t1429: f64, t9550: f64, t9554: f64, t9557: f64, t9560: f64, t9564: f64, t9569: f64, t9571: f64, t9575: f64, t9577: f64, t9579: f64, t9582: f64, t9584: f64) -> (f64, f64) {
    let t10620 = 0.14896037479937677779e-1_f64 * t10619;
    let t10621 = -t10611 + 0.39722766613167140743e-1_f64 * t1429 * t10612 - t10617 + t10620 + t9550 - t9554 + t9557 + t9560 - t9564 - t9569 - t9571 - t9575 + t9577 + t9579 + t9582 - t9584;
    (t10620, t10621)
}
