//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 55 (v4rho2sigma2_11) CSE chunk 1138/1304 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part55_v4rho2sigma2_11_chunk1138<F: Float>(t23383: F, t8020: F, t2122: F, t8034: F, t5052: F, t7299: F, t225: F, t27419: F, t27805: F, t27424: F, t27422: F, t111: F, t27370: F) -> (F, F, F, F, F, F, F, F) {
    let t94490 = t8020 * t23383;
    let t94514 = t8034 * t2122;
    let t94558 = t7299 * t5052;
    let t94656 = t27419 * t225;
    let t95836 = t27805 * t225;
    let t95899 = t27424 * t225;
    let t95902 = t27422 * t225;
    let t96238 = t27370 * t111;
    (t94490, t94514, t94558, t94656, t95836, t95899, t95902, t96238)
}
