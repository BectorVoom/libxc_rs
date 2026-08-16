//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 32 (v4rho3sigma_8) CSE chunk 2226/2369 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk2226<F: Float>(t22893: F, t23164: F, t28345: F, t23153: F, t5544: F, t6552: F, t6637: F, t16662: F, t6638: F, t28329: F, t16927: F, t87052: F, t87529: F) -> (F, F, F, F, F) {
    let t98345 = t23164 * t22893 * t28345;
    let t98349 = t6552 * t6637 * t23153 * t5544;
    let t98353 = t6552 * t6637 * t6638 * t16662;
    let t98356 = t23164 * t22893 * t28329;
    let t98359 = t87052 * t87529 * t16927;
    (t98345, t98349, t98353, t98356, t98359)
}
