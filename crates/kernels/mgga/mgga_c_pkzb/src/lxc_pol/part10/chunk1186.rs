//! MGGA_C_PKZB lxc pol — lxc_pol part 10 (v4rho4_2) CSE chunk 1186/1420 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part10_v4rho4_2_chunk1186<F: Float>(t6861: F, t6966: F, t167: F, t168: F, t17033: F, t16421: F, t2591: F, t5257: F, t6958: F, t16399: F, t6908: F, t1702: F, t6930: F, t1769: F, t7005: F, t1037: F, t16406: F) -> (F, F, F, F, F, F, F, F) {
    let t20057 = t6966 * t6861;
    let t20060 = t167 * t168 * t17033;
    let t20065 = t16421 * t168 * t2591;
    let t20085 = t5257 * t6958;
    let t20118 = t16399 * t6908;
    let t20121 = t1702 * t6930;
    let t20127 = t1769 * t7005;
    let t20155 = t16406 * t1037;
    (t20057, t20060, t20065, t20085, t20118, t20121, t20127, t20155)
}
