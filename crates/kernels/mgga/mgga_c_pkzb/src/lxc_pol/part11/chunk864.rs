//! MGGA_C_PKZB lxc pol — lxc_pol part 11 (v4rho4_3) CSE chunk 864/1208 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part11_v4rho4_3_chunk864<F: Float>(t3820: F, t889: F, t1209: F, t2252: F, t2291: F, t2296: F, t3793: F, t3796: F, t3807: F, t3823: F, t6266: F, t6300: F, t6308: F, t8102: F, t863: F, t882: F, t891: F, t9930: F, t9959: F, t9964: F, t9974: F, t9978: F, t9980: F) -> (F, F) {
    let t9981 = t3820 * t889;
    let t9984 = -0.11696447245269292414e1 * t6266 * t3807 + 0.5848223622634646207e0 * t2291 * t3820 + 0.5848223622634646207e0 * t882 * t9930 + 0.17315859105681463759e2 * t6300 * t3823 + 1.0 * t2252 * t3793 + 1.0 * t863 * t9959 + 0.32163958997385070134e2 * t6308 * t3796 + 0.5848223622634646207e0 * t9964 * t891 + 0.11696447245269292414e1 * t8102 * t1209 - 0.19751673498613801407e-1 * t9974 - t9978 - t9980 - 0.11696447245269292414e1 * t2296 * t9981;
    (t9981, t9984)
}
