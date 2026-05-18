//! GGA_C_GAPLOC lxc pol — lxc_pol part 27 (v4rho2sigma2_10) CSE chunk 888/1468 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part27_v4rho2sigma2_10_chunk888<F: Float>(t2927: F, t779: F, t3049: F, t702: F, t740: F, t8637: F, t1024: F, t2042: F, t1035: F, t1881: F, t1029: F, t2095: F) -> (F, F, F, F, F, F) {
    let t8912 = t779 * t2927;
    let t8919 = t3049 * t702;
    let t8926 = t8637 * t740;
    let t8929 = t2042 * t1024;
    let t8932 = t1035 * t1881;
    let t8939 = t2095 * t1029;
    (t8912, t8919, t8926, t8929, t8932, t8939)
}
