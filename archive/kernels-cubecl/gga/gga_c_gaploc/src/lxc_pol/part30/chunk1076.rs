//! GGA_C_GAPLOC lxc pol — lxc_pol part 30 (v4rho2sigma2_13) CSE chunk 1076/1436 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part30_v4rho2sigma2_13_chunk1076<F: Float>(t24644: F, t2610: F, t24536: F, t739: F, t1890: F, t10007: F, t8669: F, t1858: F, t2925: F, t203: F, t7861: F, t1323: F, t986: F) -> (F, F, F, F, F, F, F) {
    let t25289 = t2610 * t24644;
    let t25331 = t739 * t24536;
    let t25335 = t1890 * t24536;
    let t25359 = t10007 * t8669;
    let t25462 = t1858 * t2925;
    let t25556 = t203 * t7861;
    let t25574 = t986 * t1323;
    (t25289, t25331, t25335, t25359, t25462, t25556, t25574)
}
