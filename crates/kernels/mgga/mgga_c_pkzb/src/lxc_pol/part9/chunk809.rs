//! MGGA_C_PKZB lxc pol — lxc_pol part 9 (v4rho4_1) CSE chunk 809/1336 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part9_v4rho4_1_chunk809<F: Float>(t5796: F, t684: F, t664: F, t1897: F, t662: F, t212: F) -> (F, F, F, F) {
    let t5797 = t5796 * t684;
    let t5799 = F::new(1.0) * t664 * t5797;
    let t5801 = F::new(1.0) / t1897 / t662;
    let t5802 = t212 * t5801;
    (t5797, t5799, t5801, t5802)
}
