//! MGGA_C_KCIS lxc pol — lxc_pol part 4 (v3rho3_1) CSE chunk 642/1420 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part4_v3rho3_1_chunk642<F: Float>(t1262: F, t922: F, t3515: F, t1071: F, t421: F, t2630: F, t1252: F, t1253: F, t2635: F, t1258: F, t420: F, t287: F) -> (F, F, F, F, F, F, F, F) {
    let t3516 = t922 * t1262;
    let t3517 = t3515 * t3516;
    let t3520 = t421 * t1071;
    let t3521 = t3520 * t2630;
    let t3522 = t1252 * t3521;
    let t3525 = t1253 * t2635;
    let t3526 = t1252 * t3525;
    let t3530 = F::new(1.0) / t1258 / t420;
    let t3531 = t287 * t3530;
    (t3516, t3517, t3521, t3522, t3525, t3526, t3530, t3531)
}
