//! MGGA_C_KCIS lxc pol — lxc_pol part 23 (v4rho3sigma_5) CSE chunk 1109/1177 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part23_v4rho3sigma_5_chunk1109<F: Float>(t1464: F, t2038: F, t4124: F, t98310: F, t1386: F, t16962: F, t1307: F, t1380: F, t1943: F, t94228: F, t1394: F, t27364: F, t5655: F, t17005: F, t7923: F, t17010: F, t4153: F) -> (F, F, F, F, F, F) {
    let t98313 = t1464 * t98310 * t2038 * t4124;
    let t98315 = t16962 * t1386;
    let t98322 = t94228 * t1943 * t1380 * t1307;
    let t98328 = t1394 * t27364 * t5655;
    let t98331 = t1394 * t7923 * t17005;
    let t98334 = t4153 * t7923 * t17010;
    (t98313, t98315, t98322, t98328, t98331, t98334)
}
