//! MGGA_C_KCIS lxc pol — lxc_pol part 4 (v3rho3_1) CSE chunk 1186/1239 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part4_v3rho3_1_chunk1186<F: Float>(t16653: F, t4261: F, t4260: F, t492: F, t6015: F, t4262: F, t1539: F, t5999: F, t16665: F, t6011: F, t6010: F, t2042: F, t4256: F, t4255: F, t2035: F, t4270: F) -> (F, F, F, F, F, F) {
    let t17409 = t4261 * t16653;
    let t17410 = t4260 * t17409;
    let t17412 = t6015 * t492;
    let t17413 = t17412 * t4262;
    let t17415 = t5999 * t1539;
    let t17417 = t6011 * t16665;
    let t17418 = t6010 * t17417;
    let t17420 = t2042 * t4256;
    let t17421 = t4255 * t17420;
    let t17423 = t2035 * t4270;
    (t17410, t17413, t17415, t17418, t17421, t17423)
}
