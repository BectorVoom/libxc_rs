//! MGGA_C_KCIS lxc pol — lxc_pol part 23 (v4rho3sigma_5) CSE chunk 1061/1177 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part23_v4rho3sigma_5_chunk1061<F: Float>(t16968: F, t3717: F, t1380: F, t1385: F, t1377: F, t1593: F, t27371: F, t27369: F, t52613: F, t7908: F, t7910: F, t27376: F, t27459: F, t16937: F, t27454: F, t27380: F, t4142: F) -> (F, F, F, F, F, F, F, F, F) {
    let t94228 = t16968 * t3717;
    let t94229 = t1380 * t1385;
    let t94246 = t1593 * t1377;
    let t94247 = t94246 * t27371;
    let t94248 = t27369 * t94247;
    let t94287 = t7908 * t52613 * t7910;
    let t94289 = t27459 * t27376;
    let t94310 = t7908 * t16937 * t27454;
    let t94319 = t4142 * t27380;
    (t94228, t94229, t94246, t94247, t94248, t94287, t94289, t94310, t94319)
}
