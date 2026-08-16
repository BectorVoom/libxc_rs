//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 29 (v4rho3sigma_5) CSE chunk 2134/2357 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk2134<F: Float>(t87221: F, t87259: F, t87286: F, t87324: F, t87377: F, t87415: F, t87455: F, t87509: F, t22986: F, t25249: F, t2679: F, t6646: F) -> (F, F) {
    let t87512 = t87221 + t87259 + t87286 + t87324 + t87377 + t87415 + t87455 + t87509;
    let t87517 = t22986 * t6646 * t25249 * t2679;
    (t87512, t87517)
}
