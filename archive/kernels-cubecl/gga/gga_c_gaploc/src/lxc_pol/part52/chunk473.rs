//! GGA_C_GAPLOC lxc pol — lxc_pol part 52 (v4rhosigma3_17) CSE chunk 473/1013 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part52_v4rhosigma3_17_chunk473<F: Float>(t2021: F, t6110: F, t1858: F, t935: F, t1890: F, t7291: F, t5241: F, t739: F, t7068: F, t2530: F, t325: F, t883: F) -> (F, F, F, F, F, F, F, F, F) {
    let t7630 = t2021 * t6110;
    let t7634 = t1858 * t935;
    let t7659 = t1890 * t7291;
    let t7663 = t5241 * t7291;
    let t7667 = t739 * t7291;
    let t7671 = t739 * t7068;
    let t7675 = t1890 * t7068;
    let t7696 = t1890 * t2530;
    let t7784 = t883 * t325;
    (t7630, t7634, t7659, t7663, t7667, t7671, t7675, t7696, t7784)
}
