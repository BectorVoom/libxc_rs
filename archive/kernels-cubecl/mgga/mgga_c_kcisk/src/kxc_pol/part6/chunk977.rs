//! MGGA_C_KCISK kxc pol — kxc_pol part 6 (v3rho3_3) CSE chunk 977/1086 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_kxc_pol_part6_v3rho3_3_chunk977<F: Float>(t30212: F, t3796: F, t3482: F, t2075: F, t8251: F, t3484: F, t5886: F, t7907: F, t1411: F, t2236: F, t25308: F, t2231: F, t8072: F) -> (F, F, F, F, F) {
    let t30213 = t3796 * t30212;
    let t30214 = t3482 * t30213;
    let t30216 = t8251 * t2075;
    let t30217 = t3484 * t30216;
    let t30218 = t3482 * t30217;
    let t30220 = t5886 * t7907;
    let t30221 = t1411 * t30220;
    let t30223 = t25308 * t2236;
    let t30224 = t1411 * t30223;
    let t30226 = t8072 * t2231;
    (t30214, t30218, t30221, t30224, t30226)
}
