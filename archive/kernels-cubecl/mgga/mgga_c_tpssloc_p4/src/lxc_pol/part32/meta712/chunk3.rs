//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 32 (v4rho3sigma_8) CSE chunk 2236/2369 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk2236<F: Float>(t1484: F, t6552: F, t6637: F, t87586: F, t1509: F, t7510: F, t1888: F, t232: F, t58166: F, t6646: F, t16815: F, t22986: F, t2647: F) -> (F, F, F, F) {
    let t98520 = t6552 * t6637 * t87586 * t1484;
    let t98524 = t7510 * t1509;
    let t98530 = t1888 * t6646 * t58166 * t232;
    let t98534 = t22986 * t6646 * t16815 * t2647;
    (t98520, t98524, t98530, t98534)
}
