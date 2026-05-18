//! GGA_C_ACGGAP lxc pol — lxc_pol part 15 (v4rho3sigma_7) CSE chunk 948/1278 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part15_v4rho3sigma_7_chunk948<F: Float>(t2131: F, t2132: F, t2217: F, t847: F, t7990: F, t8065: F, t32165: F, t8104: F, t32181: F, t33232: F, t4210: F, t3035: F, t3923: F, t633: F) -> (F, F, F, F, F) {
    let t33278 = t2131 * t2132 * t2217 * t847;
    let t33281 = t7990 * t8065;
    let t33284 = F::new(0.26020884564615598386e1) * t32165 * t8104;
    let t33286 = t32181 * t33232 * t4210;
    let t33293 = F::new(0.39512695097613069591e1) * t3035 * t633 * t3923;
    (t33278, t33281, t33284, t33286, t33293)
}
