//! MGGA_C_PKZB lxc pol — lxc_pol part 11 (v4rho4_3) CSE chunk 1127/1340 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part11_v4rho4_3_chunk1127<F: Float>(t16399: F, t8916: F, t164: F, t8888: F, t5257: F, t8906: F, t6966: F, t8911: F, t17053: F, t3418: F, t8897: F, t1769: F, t8823: F) -> (F, F, F, F, F, F, F) {
    let t24298 = t16399 * t8916;
    let t24300 = t8888 * t164;
    let t24320 = t5257 * t8906;
    let t24322 = t6966 * t8911;
    let t24347 = t17053 * t3418;
    let t24370 = t5257 * t8897;
    let t24381 = t1769 * t8823;
    (t24298, t24300, t24320, t24322, t24347, t24370, t24381)
}
