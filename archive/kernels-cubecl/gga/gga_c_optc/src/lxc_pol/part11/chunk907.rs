//! GGA_C_OPTC lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 907/1451 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part11_v4rho4_4_chunk907<F: Float>(t1: F, t17045: F, t297: F, t313: F, t16988: F, t7380: F, t935: F, t16225: F, t7405: F, t322: F, t7924: F, t16231: F, t865: F) -> (F, F, F, F, F, F, F, F, F) {
    let t17047 = t17045 * t1 * t297;
    let t17048 = t313 * t17047;
    let t17052 = t16988 * t7380 * t935;
    let t17053 = t313 * t17052;
    let t17056 = t7405 * t16225;
    let t17057 = t322 * t17056;
    let t17060 = t7924 * t16225;
    let t17061 = t322 * t17060;
    let t17064 = t865 * t16231;
    (t17047, t17048, t17052, t17053, t17056, t17057, t17060, t17061, t17064)
}
