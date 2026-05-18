//! MGGA_C_RMGGAC lxc pol — lxc_pol part 15 (v4rho3sigma_6) CSE chunk 965/1110 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part15_v4rho3sigma_6_chunk965<F: Float>(t1971: F, t236: F, t6130: F, t7365: F, t495: F, t7231: F, t8517: F, t9988: F, t4601: F, t9999: F, t10053: F, t25918: F) -> (F, F, F, F) {
    let t45994 = t7365 * t1971 * t236 * t6130;
    let t45999 = t8517 * t7231 * t236 * t9988 * t495;
    let t46001 = t4601 * t9999;
    let t46003 = t25918 * t10053;
    (t45994, t45999, t46001, t46003)
}
