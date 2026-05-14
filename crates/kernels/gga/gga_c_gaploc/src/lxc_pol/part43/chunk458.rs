//! GGA_C_GAPLOC lxc pol — lxc_pol part 43 (v4rhosigma3_8) CSE chunk 458/923 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part43_v4rhosigma3_8_chunk458<F: Float>(t1890: F, t8502: F, t1980: F, t2975: F, t2925: F, t296: F, t1: F, t787: F, t325: F, t3039: F, t783: F, t1022: F, t5750: F, t106: F, t316: F, t795: F) -> (F, F, F, F, F, F, F, F) {
    let t8503 = t1890 * t8502;
    let t8516 = t1980 * t2975;
    let t8519 = t296 * t2925;
    let t8520 = t8519 * t1;
    let t8521 = t787 * t8520;
    let t8528 = t325 * t2925;
    let t8556 = t3039 * t783;
    let t8604 = t5750 * t1022;
    let t8632 = t2925 * t1;
    let t8633 = t8632 * t106;
    let t8634 = t8633 * t316;
    let t8637 = t795 * t2925;
    (t8503, t8516, t8521, t8528, t8556, t8604, t8634, t8637)
}
