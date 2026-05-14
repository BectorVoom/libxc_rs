//! MGGA_C_KCISK lxc pol — lxc_pol part 3 (v3rho3_0) CSE chunk 722/938 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part3_v3rho3_0_chunk722<F: Float>(t12220: F, t2021: F, t1586: F, t12163: F, t12166: F, t12171: F, t12175: F, t12180: F, t12183: F, t12186: F, t12188: F, t12195: F, t12200: F, t12205: F, t12209: F, t12216: F, t2013: F, t2016: F, t5471: F, t5488: F, t5494: F, t5499: F, t5503: F, t782: F) -> (F,) {
    let t12221 = t2021 * t12220;
    let t12222 = t1586 * t12221;
    let t12225 = -0.17990788716177317214e-1 * t12163 + 0.53972366148531951639e-1 * t2013 * t12166 + 0.27985671336275826777e-1 * t2013 * t12171 - 0.17990788716177317214e-1 * t12175 - 0.53972366148531951639e-1 * t5471 * t5499 - 0.59969295720591057378e-2 * t12180 + 0.89953943580886586067e-2 * t12183 + 0.11993859144118211476e-1 * t12186 + 0.17990788716177317213e-1 * t12188 + 0.2698618307426597582e-1 * t5471 * t5503 + 0.35981577432354634427e-1 * t5471 * t5488 + 0.2698618307426597582e-1 * t12195 * t2016 - 0.71963154864709268855e-1 * t2013 * t12200 + 0.16191709844559585492e0 * t2013 * t12205 + 0.89953943580886586067e-2 * t2013 * t12209 - 0.53972366148531951639e-1 * t5471 * t5494 - 0.2698618307426597582e-1 * t2013 * t12216 - 0.2698618307426597582e-1 * t782 * t12222;
    (t12225,)
}
