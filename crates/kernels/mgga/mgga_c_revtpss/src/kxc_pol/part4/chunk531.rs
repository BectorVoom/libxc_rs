//! MGGA_C_REVTPSS kxc pol — kxc_pol part 4 (v3rho3_1) CSE chunk 531/1428 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_kxc_pol_part4_v3rho3_1_chunk531<F: Float>(t2398: F, t707: F, t150: F, t2389: F, t190: F, t198: F, t206: F, t890: F, t892: F, t261: F, t2258: F, t706: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t2400 = F::new(8.0) * t2398 * t707;
    let t2401 = t150 * t2389;
    let t2402 = t2401 * t190;
    let t2403 = t198 * t206;
    let t2404 = t890 * t892;
    let t2408 = t890 * t890;
    let t2410 = t261 * t261;
    let t2411 = F::new(1.0) / t2410;
    let t2414 = t190 * t2258;
    let t2416 = F::new(4.0) * t706 * t2414;
    (t2400, t2401, t2402, t2403, t2404, t2408, t2410, t2411, t2414, t2416)
}
