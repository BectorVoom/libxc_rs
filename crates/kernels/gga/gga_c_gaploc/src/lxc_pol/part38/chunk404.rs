//! GGA_C_GAPLOC lxc pol — lxc_pol part 38 (v4rhosigma3_3) CSE chunk 404/1003 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part38_v4rhosigma3_3_chunk404<F: Float>(t169: F, t3516: F, t172: F, t452: F, t3094: F, t3107: F, t3099: F, t3104: F, t3114: F, t3336: F, t471: F) -> (F, F, F, F, F) {
    let t3517 = t3516 * t169;
    let t3518 = t3517 * t172;
    let t3519 = t452 * t3518;
    let t3522 = F::new(3.0) / F::new(64.0) * t3094;
    let t3525 = t3107 / F::new(64.0);
    let t3526 = t3522 - F::new(9.0) / F::new(2048.0) * t3099 + F::new(3.0) / F::new(2048.0) * t3104 - t3525;
    let t3529 = t3526 * t471 - F::new(2.0) * t3114 + t3336 + t3522 - t3525;
    (t3517, t3518, t3519, t3526, t3529)
}
