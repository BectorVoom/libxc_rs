//! MGGA_C_PKZB lxc pol — lxc_pol part 9 (v4rho4_1) CSE chunk 807/1336 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part9_v4rho4_1_chunk807<F: Float>(t1902: F, t5771: F, t1897: F, t224: F, t212: F) -> (F, F, F) {
    let t5773 = F::cast_from(0.48245938496077605201e2_f64) * t5771 * t1902;
    let t5775 = F::cast_from(1.0_f64) / t1897 / t224;
    let t5776 = t212 * t5775;
    (t5773, t5775, t5776)
}
