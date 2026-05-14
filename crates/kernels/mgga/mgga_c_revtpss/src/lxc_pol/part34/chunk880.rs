//! MGGA_C_REVTPSS lxc pol — lxc_pol part 34 (v4rho3sigma_9) CSE chunk 880/1196 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part34_v4rho3sigma_9_chunk880<F: Float>(t1868: F, t22486: F, t5532: F, t6836: F, t1907: F, t198: F, t22483: F, t22813: F, t22925: F, t22926: F, t5536: F, t5541: F, t566: F, t9514: F, t9517: F, t9521: F, t9524: F, t9546: F, t9569: F, t9574: F, t9577: F, t9588: F) -> (F,) {
    let t23068 = t22486 * t1868;
    let t23071 = t5532 * t6836;
    let t23077 = -3.0 * t1907 * t22483 * t5541 + 6.0 * t198 * t22813 * t566 + 18.0 * t23068 * t5536 + 18.0 * t23071 * t5536 - t22925 - t22926 + t9514 - t9517 - t9521 - t9524 + t9546 + t9569 - t9574 - t9577 - t9588;
    (t23077,)
}
