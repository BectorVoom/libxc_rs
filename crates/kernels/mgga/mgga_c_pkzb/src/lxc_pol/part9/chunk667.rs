//! MGGA_C_PKZB lxc pol — lxc_pol part 9 (v4rho4_1) CSE chunk 667/1213 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part9_v4rho4_1_chunk667<F: Float>(t3147: F, t900: F, t1217: F, t2328: F, t1208: F, t2295: F, t891: F, t898: F, t3135: F, t881: F, t890: F, t2317: F, t2320: F, t889: F, t1220: F, t904: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
    let t3149 = 0.5848223622634646207e0 * t3147 * t900;
    let t3151 = 0.5848223622634646207e0 * t2328 * t1217;
    let t3152 = t2295 * t1208;
    let t3153 = t3152 * t891;
    let t3155 = 0.11696447245269292414e1 * t898 * t3153;
    let t3157 = t881 * t3135 * t890;
    let t3159 = 0.5848223622634646207e0 * t898 * t3157;
    let t3160 = t2317 * t1208;
    let t3161 = t2320 * t889;
    let t3162 = t3160 * t3161;
    let t3164 = 0.17315859105681463759e2 * t898 * t3162;
    let t3165 = t1220 * t904;
    (t3149, t3151, t3152, t3153, t3155, t3157, t3159, t3160, t3161, t3162, t3164, t3165)
}
