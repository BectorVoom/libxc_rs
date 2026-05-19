//! MGGA_C_PKZB lxc pol — lxc_pol part 11 (v4rho4_3) CSE chunk 267/1340 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part11_v4rho4_3_chunk267<F: Float>(t881: F, t889: F, t890: F, t898: F, t154: F, t386: F, t67: F, t385: F, t405: F, t52: F) -> (F, F, F, F, F) {
    let t900 = t881 * t889 * t890;
    let t902 = F::cast_from(0.5848223622634646207e0_f64) * t898 * t900;
    let t904 = t154 * t67 * t386;
    let t906 = t385 * t904 / F::new(288.0);
    let t907 = t52 * t405;
    (t900, t902, t904, t906, t907)
}
