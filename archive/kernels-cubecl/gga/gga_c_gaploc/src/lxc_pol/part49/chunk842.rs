//! GGA_C_GAPLOC lxc pol — lxc_pol part 49 (v4rhosigma3_14) CSE chunk 842/1217 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part49_v4rhosigma3_14_chunk842<F: Float>(t107: F, t33137: F, t787: F, t10012: F, t10627: F, t10892: F, t1980: F, t1858: F, t3431: F, t2101: F, t1890: F, t3487: F) -> (F, F, F, F, F, F) {
    let t33139 = t787 * t33137 * t107;
    let t33148 = t10012 * t10627;
    let t33206 = t1980 * t10892;
    let t33232 = t1858 * t3431;
    let t33285 = t2101 * t3431;
    let t33289 = t1890 * t3487;
    (t33139, t33148, t33206, t33232, t33285, t33289)
}
