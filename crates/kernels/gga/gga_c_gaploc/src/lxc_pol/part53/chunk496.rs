//! GGA_C_GAPLOC lxc pol — lxc_pol part 53 (v4rhosigma3_18) CSE chunk 496/923 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part53_v4rhosigma3_18_chunk496<F: Float>(t1429: F, t9580: F, t2372: F, t2389: F, t9554: F, t9557: F, t9560: F, t9564: F, t9565: F, t9569: F, t9571: F, t9575: F, t9577: F, t9579: F, t9308: F, t9349: F, t9388: F, t9434: F, t9475: F, t9509: F, t9551: F) -> (F, F, F) {
    let t9582 = 0.59584149919750711116e-1 * t1429 * t9580;
    let t9584 = 0.59584149919750711116e-1 * t2372 * t2389;
    let t9585 = -t9554 + t9557 + t9560 - t9564 + 0.39722766613167140743e-1 * t1429 * t9565 - t9569 - t9571 - t9575 + t9577 + t9579 + t9582 - t9584;
    let t9588 = t9308 + t9349 + t9388 + t9434 + t9475 + t9509 + t9551 + t9585;
    (t9582, t9584, t9588)
}
