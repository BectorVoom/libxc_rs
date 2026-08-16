//! MGGA_C_PKZB lxc pol — lxc_pol part 9 (v4rho4_1) CSE chunk 1072/1336 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part9_v4rho4_1_chunk1072<F: Float>(t5075: F, t512: F, t83: F, t1511: F, t5336: F, t204: F, t99: F, t4888: F, t5029: F, t4892: F, t5052: F, t496: F, t5076: F) -> (F, F, F, F, F) {
    let t16897 = t83 * t512 * t5075;
    let t16901 = t1511 * t5336;
    let t16903 = t99 * t204;
    let t16906 = F::cast_from(0.1301229756036208781e0_f64) * t16903 * t5029 * t4888;
    let t16909 = F::cast_from(0.19263893255070628431e1_f64) * t16903 * t5052 * t4892;
    let t16910 = t496 * t5076;
    (t16897, t16901, t16906, t16909, t16910)
}
