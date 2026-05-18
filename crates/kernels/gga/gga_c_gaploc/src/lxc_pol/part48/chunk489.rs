//! GGA_C_GAPLOC lxc pol — lxc_pol part 48 (v4rhosigma3_13) CSE chunk 489/1003 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part48_v4rhosigma3_13_chunk489<F: Float>(t1022: F, t1858: F, t1: F, t8773: F, t2021: F, t1890: F, t2925: F, t1033: F, t1959: F, t161: F, t2931: F, t1023: F, t1853: F) -> (F, F, F, F, F, F, F, F) {
    let t8788 = t1858 * t1022;
    let t8792 = t8773 * t1;
    let t8793 = t2021 * t8792;
    let t8802 = t1890 * t2925;
    let t8862 = t1033 * t1959;
    let t8867 = t2931 * t161;
    let t8878 = t8773 * t161;
    let t8942 = t1023 * t1853;
    (t8788, t8792, t8793, t8802, t8862, t8867, t8878, t8942)
}
