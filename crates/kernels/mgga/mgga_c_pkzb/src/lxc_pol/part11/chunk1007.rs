//! MGGA_C_PKZB lxc pol — lxc_pol part 11 (v4rho4_3) CSE chunk 1007/1340 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part11_v4rho4_3_chunk1007<F: Float>(t11100: F, t790: F, t11054: F, t11064: F, t11067: F, t1134: F, t1144: F, t307: F, t311: F, t3670: F, t3676: F, t3695: F) -> (F, F) {
    let t11101 = t790 * t11100;
    let t11104 = F::new(0.65854491829355115987e0) * t11054 * t311 - F::new(0.19756347548806534796e1) * t3670 * t1144 + F::new(0.39512695097613069591e1) * t1134 * t3676 - F::new(0.19756347548806534796e1) * t1134 * t3695 - F::new(0.39512695097613069591e1) * t307 * t11064 + F::new(0.39512695097613069591e1) * t307 * t11067 - F::new(0.65854491829355115987e0) * t307 * t11101;
    (t11101, t11104)
}
