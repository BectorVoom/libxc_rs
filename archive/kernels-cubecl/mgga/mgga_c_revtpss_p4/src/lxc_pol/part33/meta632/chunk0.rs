//! MGGA_C_REVTPSS lxc pol — lxc_pol part 33 (v4rho3sigma_8) CSE chunk 2079/2275 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk2079<F: Float>(t25387: F, t99349: F, t2470: F, t27340: F, t7063: F, t99271: F, t7060: F, t136: F, t2457: F, t7778: F, t25299: F, t25412: F, t99348: F) -> (F, F, F, F, F, F, F) {
    let t99351 = F::cast_from(0.51405703062096148812e-1_f64) * t25387 * t99349;
    let t99365 = t27340 * t2470;
    let t99366 = t25387 * t99365;
    let t99373 = t7063 * t99271;
    let t99375 = F::cast_from(0.25702851531048074406e-1_f64) * t99373 * t7060;
    let t99380 = t7778 * t136 * t2457;
    let t99381 = t25299 * t99380;
    let t99389 = t99348 * t25412;
    (t99351, t99365, t99366, t99375, t99380, t99381, t99389)
}
