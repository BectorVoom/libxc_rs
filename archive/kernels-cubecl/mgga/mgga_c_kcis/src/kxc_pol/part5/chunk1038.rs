//! MGGA_C_KCIS kxc pol — kxc_pol part 5 (v3rho3_2) CSE chunk 1038/1419 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_kxc_pol_part5_v3rho3_2_chunk1038<F: Float>(t13396: F, t1392: F, t86: F, t5782: F, t2007: F, t3245: F, t1014: F, t5891: F, t3728: F, t5629: F, t14249: F, t5446: F) -> (F, F, F, F, F, F, F) {
    let t15967 = t86 * t13396 * t1392;
    let t15968 = t15967 * t5782;
    let t15983 = t3245 * t2007;
    let t15986 = t1014 * t5891;
    let t15987 = F::cast_from(0.88437037037037037034e-2_f64) * t15986;
    let t15988 = t3728 * t5629;
    let t15989 = F::cast_from(0.33163888888888888888e-2_f64) * t15988;
    let t15994 = t14249 * t5446;
    (t15968, t15983, t15986, t15987, t15988, t15989, t15994)
}
