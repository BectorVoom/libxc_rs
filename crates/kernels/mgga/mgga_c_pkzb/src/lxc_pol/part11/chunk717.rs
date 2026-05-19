//! MGGA_C_PKZB lxc pol — lxc_pol part 11 (v4rho4_3) CSE chunk 717/1340 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part11_v4rho4_3_chunk717<F: Float>(t5296: F, t618: F, t144: F, t174: F, t46: F, t123: F, t475: F, t574: F, t550: F, t1667: F, t1670: F, t1497: F, t1613: F, t542: F) -> (F, F, F, F, F, F, F) {
    let t5297 = t5296 * t618;
    let t5304 = F::new(1.0) / t174 / t144;
    let t5305 = t5304 * t46;
    let t5322 = t475 * t574 * t123;
    let t5324 = F::cast_from(0.56968947174242584612e-3_f64) * t550 * t5322;
    let t5325 = t1670 * t1667;
    let t5331 = t1613 * t1497 * t542;
    (t5297, t5304, t5305, t5322, t5324, t5325, t5331)
}
