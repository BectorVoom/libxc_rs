//! MGGA_C_PKZB lxc pol — lxc_pol part 11 (v4rho4_3) CSE chunk 1068/1340 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part11_v4rho4_3_chunk1068<F: Float>(t204: F, t99: F, t4888: F, t5029: F, t4892: F, t5052: F, t496: F, t5076: F, t123: F, t1697: F, t475: F, t550: F) -> (F, F, F, F) {
    let t16903 = t99 * t204;
    let t16906 = F::new(0.1301229756036208781e0) * t16903 * t5029 * t4888;
    let t16909 = F::new(0.19263893255070628431e1) * t16903 * t5052 * t4892;
    let t16910 = t496 * t5076;
    let t16915 = F::new(0.18989649058080861537e-2) * t550 * t475 * t1697 * t123;
    (t16906, t16909, t16910, t16915)
}
