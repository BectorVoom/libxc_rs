//! MGGA_C_PKZB lxc pol — lxc_pol part 9 (v4rho4_1) CSE chunk 1152/1336 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part9_v4rho4_1_chunk1152<F: Float>(t17043: F, t7091: F, t6861: F, t6966: F, t167: F, t168: F, t17033: F, t16421: F, t2591: F, t1034: F, t16425: F, t5373: F) -> (F, F, F, F, F) {
    let t20037 = t17043 * t7091;
    let t20057 = t6966 * t6861;
    let t20060 = t167 * t168 * t17033;
    let t20065 = t16421 * t168 * t2591;
    let t20067 = t1034 * t16425 * t5373;
    (t20037, t20057, t20060, t20065, t20067)
}
