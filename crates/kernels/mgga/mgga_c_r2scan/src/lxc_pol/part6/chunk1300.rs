//! MGGA_C_R2SCAN lxc pol — lxc_pol part 6 (v4rho4_1) CSE chunk 1300/1462 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part6_v4rho4_1_chunk1300<F: Float>(t2133: F, t2294: F, t8002: F, t2568: F, t3433: F, t19852: F, t6118: F, t7535: F, t19881: F, t19884: F, t19886: F, t19892: F, t19895: F, t19904: F, t19907: F, t19929: F, t19932: F, t19948: F, t19951: F) -> (F,) {
    let t24515 = t2133 * t2294 * t8002;
    let t24521 = t3433 * t2568;
    let t24522 = t19852 * t24521;
    let t24523 = 0.57131963037208741166e-1 * t24522;
    let t24524 = t6118 * t7535;
    let t24526 = -0.20958572791407956061e0 * t19881 + 0.34930954652346593433e-1 * t19884 - 0.5141876673348786705e0 * t19886 - 0.4075278042773769234e0 * t19892 - 0.12225834128321307702e1 * t19895 + t19904 - 0.87816964854445047167e-1 * t19907 - 0.69345773920434148506e0 * t24515 + 0.25426783770825854452e1 * t19929 + 0.76280351312477563356e1 * t19932 - 0.34672886960217074253e0 * t19948 - 0.10401866088065122276e1 * t19951 + t24523 + 0.76830240467580968651e0 * t24524;
    (t24526,)
}
