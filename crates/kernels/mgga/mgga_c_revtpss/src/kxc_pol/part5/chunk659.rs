//! MGGA_C_REVTPSS kxc pol — kxc_pol part 5 (v3rho3_2) CSE chunk 659/1286 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_kxc_pol_part5_v3rho3_2_chunk659<F: Float>(t4365: F, t4366: F, t4364: F, t1544: F, t854: F, t236: F, t807: F, t2498: F, t2518: F, t2522: F, t2526: F, t2562: F, t2569: F, t2579: F, t2587: F, t2610: F, t4300: F, t4301: F, t4304: F) -> (F, F, F, F, F) {
    let t4367 = t4365 * t4366;
    let t4368 = t4364 * t4367;
    let t4371 = t854 * t1544;
    let t4372 = t236 * t4371;
    let t4373 = t807 * t4372;
    let t4376 = t4300 - t2569 + t2579 + t2587 - t2522 - t2498 - t2518 - t4301 + t2526 + t2610 - t4304 - t2562;
    (t4368, t4371, t4372, t4373, t4376)
}
