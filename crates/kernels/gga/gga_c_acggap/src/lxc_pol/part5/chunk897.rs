//! GGA_C_ACGGAP lxc pol — lxc_pol part 5 (v4rho4_2) CSE chunk 897/1191 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part5_v4rho4_2_chunk897<F: Float>(t1588: F, t3228: F, t1008: F, t5232: F, t1049: F, t4801: F, t1483: F, t3143: F, t13747: F, t503: F, t1068: F, t1072: F, t1427: F, t3114: F, t3126: F, t4794: F, t4796: F, t576: F) -> (F, F, F, F, F, F, F) {
    let t16205 = t3228 * t1588;
    let t16207 = t1008 * t5232;
    let t16209 = t1049 * t4801;
    let t16211 = t3143 * t1483;
    let t16213 = t13747 * t503;
    let t16230 = t1068 * t3114 * t1072 * t1427 * t3126;
    let t16233 = t576 * t4794 * t4796;
    (t16205, t16207, t16209, t16211, t16213, t16230, t16233)
}
