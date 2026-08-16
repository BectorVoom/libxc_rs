//! GGA_C_GAPLOC lxc pol — lxc_pol part 36 (v4rhosigma3_1) CSE chunk 533/1029 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part36_v4rhosigma3_1_chunk533(t6696: f64, t901: f64, t6700: f64, t3162: f64, t549: f64, t1429: f64, t2372: f64, t2389: f64, t9554: f64, t9557: f64, t9560: f64, t9564: f64, t9565: f64, t9569: f64, t9571: f64, t9575: f64) -> (f64, f64, f64, f64, f64) {
    let t9577 = 0.29792074959875355558e-1_f64 * t6696 * t901;
    let t9579 = 0.29792074959875355558e-1_f64 * t6700 * t901;
    let t9580 = t549 * t3162;
    let t9582 = 0.59584149919750711116e-1_f64 * t1429 * t9580;
    let t9584 = 0.59584149919750711116e-1_f64 * t2372 * t2389;
    let t9585 = -t9554 + t9557 + t9560 - t9564 + 0.39722766613167140743e-1_f64 * t1429 * t9565 - t9569 - t9571 - t9575 + t9577 + t9579 + t9582 - t9584;
    (t9577, t9579, t9582, t9584, t9585)
}
