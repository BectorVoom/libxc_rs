//! GGA_C_GAPLOC lxc pol — lxc_pol part 33 (v4rho2sigma2_16) CSE chunk 987/1294 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part33_v4rho2sigma2_16_chunk987<F: Float>(t1415: F, t8265: F, t2897: F, t4360: F, t1: F, t25694: F, t544: F, t7905: F, t9439: F, t1570: F, t2754: F, t8247: F, t7892: F, t9448: F, t4250: F, t986: F) -> (F, F, F, F, F, F, F, F, F) {
    let t26763 = t1415 * t8265;
    let t26773 = t4360 * t2897;
    let t26822 = t544 * t25694 * t1;
    let t26922 = t9439 * t7905;
    let t26938 = t1570 * t2754;
    let t26984 = t1415 * t8247;
    let t27003 = t9439 * t7892;
    let t27007 = t9448 * t7892;
    let t27071 = t4250 * t986;
    (t26763, t26773, t26822, t26922, t26938, t26984, t27003, t27007, t27071)
}
