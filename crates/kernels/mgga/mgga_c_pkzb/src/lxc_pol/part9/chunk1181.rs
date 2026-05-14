//! MGGA_C_PKZB lxc pol — lxc_pol part 9 (v4rho4_1) CSE chunk 1181/1213 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part9_v4rho4_1_chunk1181<F: Float>(t8040: F, t881: F, t2317: F, t3113: F, t1201: F, t6230: F, t8227: F, t862: F, t2278: F, t3080: F, t1189: F, t6287: F, t1197: F, t1209: F, t18799: F, t18860: F, t22309: F, t2274: F, t2282: F, t2291: F, t2313: F, t2321: F, t3083: F, t3116: F, t3136: F, t6263: F, t6279: F, t6283: F, t6291: F, t6294: F, t8099: F, t8102: F, t8115: F, t872: F, t882: F, t890: F, t891: F) -> (F,) {
    let t22740 = t8040 * t881;
    let t22745 = t3113 * t2317;
    let t22750 = t1201 * t6230;
    let t22757 = t8227 * t862;
    let t22762 = t3080 * t2278;
    let t22767 = t1189 * t6287;
    let t22772 = 0.17544670867903938621e1 * t2291 * t8099 + 0.5848223622634646207e0 * t882 * t22309 * t890 + 0.17544670867903938621e1 * t22740 * t891 + 0.17544670867903938621e1 * t8102 * t2313 + 0.51947577317044391276e2 * t22745 * t2321 + 0.5848223622634646207e0 * t3116 * t6279 + 0.10254018858216406658e4 * t22750 * t6283 + 0.5848223622634646207e0 * t18860 * t1209 + 0.17544670867903938621e1 * t6294 * t3136 + 3.0 * t22757 * t872 + 3.0 * t8115 * t2274 + 0.96491876992155210402e2 * t22762 * t2282 + 1.0 * t3083 * t6263 + 0.2069040516770936012e4 * t22767 * t6291 + 1.0 * t18799 * t1197;
    (t22772,)
}
