//! GGA_C_GAPC lxc pol — lxc_pol part 23 (v4rho2sigma2_2) CSE chunk 545/1126 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part23_v4rho2sigma2_2_chunk545<F: Float>(t3371: F, t3375: F, t3103: F, t320: F, t2636: F, t876: F, t2972: F, t916: F, t128: F, t830: F, t1086: F, t2619: F, t2979: F, t2982: F, t787: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t3376 = t3371 * t3375;
    let t3378 = t320 * t3103;
    let t3379 = t2636 * t876;
    let t3380 = t3378 * t3379;
    let t3382 = t916 * t2972;
    let t3383 = t830 * t128;
    let t3384 = t1086 * t3383;
    let t3385 = t3382 * t3384;
    let t3387 = t2619 * t2979;
    let t3388 = t2982 * t787;
    (t3376, t3378, t3379, t3380, t3382, t3383, t3384, t3385, t3387, t3388)
}
