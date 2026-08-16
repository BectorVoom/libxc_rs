//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 21 (v4rho4_2) CSE chunk 1987/3221 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1987<F: Float>(t11697: F, t4949: F, t3577: F, t3431: F, t4729: F, t1174: F, t1177: F, t14749: F, t14753: F, t14744: F, t1011: F, t15031: F) -> (F, F, F, F, F, F, F, F) {
    let t15572 = t11697 * t4949;
    let t15574 = t3577 * t15572 / F::cast_from(3456.0_f64);
    let t15578 = t3431 * t4729;
    let t15580 = t1174 * t15578 / F::cast_from(216.0_f64);
    let t15581 = t1177 * t14749;
    let t15584 = t1177 * t14753;
    let t15587 = t1177 * t14744;
    let t15590 = t15031 * t1011;
    (t15572, t15574, t15578, t15580, t15581, t15584, t15587, t15590)
}
