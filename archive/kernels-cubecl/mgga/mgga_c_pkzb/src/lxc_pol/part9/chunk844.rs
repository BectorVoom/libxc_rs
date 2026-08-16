//! MGGA_C_PKZB lxc pol — lxc_pol part 9 (v4rho4_1) CSE chunk 844/1336 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part9_v4rho4_1_chunk844<F: Float>(t6088: F, t6090: F, t6093: F, t6108: F, t352: F, t2295: F, t2313: F, t889: F, t898: F, t2288: F, t237: F) -> (F, F, F, F, F) {
    let t6110 = -t6088 + F::cast_from(0.71233333333333333332e-1_f64) * t6090 - F::cast_from(0.53424999999999999999e-1_f64) * t6093 + F::cast_from(0.53425e-1_f64) * t6108;
    let t6112 = F::cast_from(0.621814e-1_f64) * t6110 * t352;
    let t6114 = t2295 * t889 * t2313;
    let t6116 = F::cast_from(0.35089341735807877242e1_f64) * t898 * t6114;
    let t6117 = t237 * t2288;
    (t6110, t6112, t6114, t6116, t6117)
}
