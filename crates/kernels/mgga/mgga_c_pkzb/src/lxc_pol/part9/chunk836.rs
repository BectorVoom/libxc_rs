//! MGGA_C_PKZB lxc pol — lxc_pol part 9 (v4rho4_1) CSE chunk 836/1336 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part9_v4rho4_1_chunk836<F: Float>(t6054: F, t790: F, t2112: F, t2120: F, t2146: F, t307: F, t311: F, t5990: F, t6002: F, t6006: F, t786: F, t800: F) -> (F, F) {
    let t6055 = t790 * t6054;
    let t6058 = F::new(0.65854491829355115987e0) * t5990 * t311 - F::new(0.19756347548806534796e1) * t2112 * t800 + F::new(0.39512695097613069591e1) * t786 * t2120 - F::new(0.19756347548806534796e1) * t786 * t2146 - F::new(0.39512695097613069591e1) * t307 * t6002 + F::new(0.39512695097613069591e1) * t307 * t6006 - F::new(0.65854491829355115987e0) * t307 * t6055;
    (t6055, t6058)
}
