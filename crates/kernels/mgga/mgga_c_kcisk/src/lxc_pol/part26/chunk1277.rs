//! MGGA_C_KCISK lxc pol — lxc_pol part 26 (v4rho3sigma_6) CSE chunk 1277/1407 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part26_v4rho3sigma_6_chunk1277<F: Float>(t32173: F, t33373: F, t13485: F, t32087: F, t33445: F, t32176: F, t3783: F, t394: F, t470: F, t12829: F, t1328: F, t33383: F, t3969: F, t32042: F, t33384: F, t1308: F, t2158: F, t3491: F) -> (F, F, F, F, F, F, F, F) {
    let t114001 = 0.69444444444444444446e-2 * t33373 * t32173;
    let t114004 = 0.23148148148148148148e-2 * t32087 * t13485 * t33445;
    let t114011 = 0.69444444444444444446e-2 * t33373 * t32176;
    let t114021 = t3783 * t394 * t470;
    let t114038 = t1328 * t12829;
    let t114059 = t33383 * t3969;
    let t114062 = t33384 * t32042;
    let t114075 = t3491 * t2158 * t1308;
    (t114001, t114004, t114011, t114021, t114038, t114059, t114062, t114075)
}
