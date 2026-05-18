//! MGGA_C_KCIS lxc pol — lxc_pol part 4 (v3rho3_1) CSE chunk 1237/1420 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part4_v3rho3_1_chunk1237<F: Float>(t1464: F, t15960: F, t3738: F, t5876: F, t13396: F, t1392: F, t86: F, t5782: F, t4177: F, t5752: F, t1394: F, t2001: F, t4124: F) -> (F, F, F, F, F) {
    let t15961 = t1464 * t15960;
    let t15963 = t3738 * t5876;
    let t15964 = t1464 * t15963;
    let t15967 = t86 * t13396 * t1392;
    let t15968 = t15967 * t5782;
    let t15970 = t5752 * t4177;
    let t15971 = t1394 * t15970;
    let t15973 = t2001 * t4124;
    (t15961, t15964, t15968, t15971, t15973)
}
