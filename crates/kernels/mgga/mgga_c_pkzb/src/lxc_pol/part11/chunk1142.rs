//! MGGA_C_PKZB lxc pol — lxc_pol part 11 (v4rho4_3) CSE chunk 1142/1340 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part11_v4rho4_3_chunk1142<F: Float>(t3604: F, t5493: F, t1937: F, t3559: F, t663: F, t9343: F, t1979: F, t9203: F, t1940: F, t9493: F, t1915: F, t5728: F, t759: F) -> (F, F, F, F, F, F, F) {
    let t26211 = t3604 * t5493;
    let t26224 = t3559 * t1937;
    let t26283 = t9343 * t663;
    let t26323 = t9203 * t1979;
    let t26336 = t9493 * t1940;
    let t26357 = t3559 * t1915;
    let t26387 = t5728 * t759;
    (t26211, t26224, t26283, t26323, t26336, t26357, t26387)
}
