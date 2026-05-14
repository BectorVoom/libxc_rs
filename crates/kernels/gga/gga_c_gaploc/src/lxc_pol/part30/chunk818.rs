//! GGA_C_GAPLOC lxc pol — lxc_pol part 30 (v4rho2sigma2_13) CSE chunk 818/1268 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part30_v4rho2sigma2_13_chunk818<F: Float>(t836: F, t8720: F, t568: F, t1880: F, t2958: F, t1445: F, t2949: F, t2950: F, t4614: F, t1457: F, t1035: F, t2052: F, t107: F, t2931: F, t2021: F, t1858: F, t3038: F) -> (F, F, F, F, F, F, F, F, F) {
    let t8721 = t836 * t8720;
    let t8722 = t568 * t8721;
    let t8725 = t2958 * t1880;
    let t8726 = t1445 * t8725;
    let t8729 = t2949 * t1880;
    let t8730 = t1445 * t8729;
    let t8733 = t4614 * t2950;
    let t8738 = t1457 * t8729;
    let t8741 = t2052 * t1035;
    let t8748 = t2931 * t107;
    let t8749 = t2021 * t8748;
    let t8752 = t1858 * t3038;
    (t8722, t8726, t8729, t8730, t8733, t8738, t8741, t8749, t8752)
}
