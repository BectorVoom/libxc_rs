//! MGGA_C_KCIS lxc pol — lxc_pol part 23 (v4rho3sigma_5) CSE chunk 1247/1323 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part23_v4rho3sigma_5_chunk1247<F: Float>(t16836: F, t3717: F, t27357: F, t5440: F, t28347: F, t94246: F, t27369: F, t1464: F, t28360: F, t94216: F, t27364: F, t28382: F) -> (F, F, F, F, F) {
    let t98359 = t16836 * t3717;
    let t98361 = t98359 * t5440 * t27357;
    let t98364 = t94246 * t28347;
    let t98365 = t27369 * t98364;
    let t98370 = t1464 * t94216 * t28360;
    let t98373 = t1464 * t27364 * t28382;
    (t98361, t98364, t98365, t98370, t98373)
}
