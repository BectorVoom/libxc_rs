//! MGGA_C_PKZB lxc pol — lxc_pol part 9 (v4rho4_1) CSE chunk 1071/1336 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part9_v4rho4_1_chunk1071<F: Float>(t1548: F, t1626: F, t1485: F, t1508: F, t1531: F, t1499: F, t126: F, t82: F, t94: F, t98: F, t501: F, t5175: F) -> (F, F, F, F, F) {
    let t16882 = t1548 * t1626;
    let t16886 = F::cast_from(0.12842595503380418954e1_f64) * t1531 * t1485 * t1508;
    let t16889 = F::cast_from(0.43374325201206959368e-1_f64) * t1531 * t1485 * t1499;
    let t16893 = F::new(24.0) * t82 * t94 * t98 * t126;
    let t16894 = t501 * t5175;
    (t16882, t16886, t16889, t16893, t16894)
}
