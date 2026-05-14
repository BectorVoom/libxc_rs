//! MGGA_C_KCIS lxc pol — lxc_pol part 4 (v3rho3_1) CSE chunk 906/1239 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part4_v3rho3_1_chunk906<F: Float>(t3025: F, t3034: F, t110: F, t1263: F, t1251: F, t25: F, t3612: F, t3483: F, t68: F, t414: F, t1258: F, t3490: F, t3504: F, t3533: F, t1259: F, t2888: F) -> (F, F, F, F, F, F, F, F) {
    let t10974 = t3025 * t3034;
    let t10989 = t110 * t1263;
    let t10990 = t1251 * t10989;
    let t10992 = t25 * t3612;
    let t10993 = t1251 * t10992;
    let t10995 = t3483 * t68;
    let t10996 = t414 * t10995;
    let t10999 = t1258 * t1258;
    let t11000 = 1.0 / t10999;
    let t11009 = t3490 * t3504;
    let t11013 = t25 * t3533;
    let t11014 = t1251 * t11013;
    let t11020 = t2888 * t1259;
    (t10974, t10990, t10993, t10996, t11000, t11009, t11014, t11020)
}
