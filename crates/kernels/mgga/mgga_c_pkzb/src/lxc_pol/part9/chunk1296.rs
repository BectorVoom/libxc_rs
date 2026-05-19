//! MGGA_C_PKZB lxc pol — lxc_pol part 9 (v4rho4_1) CSE chunk 1296/1336 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part9_v4rho4_1_chunk1296<F: Float>(t1197: F, t1209: F, t18799: F, t18860: F, t22309: F, t2274: F, t22740: F, t22745: F, t22750: F, t22757: F, t22762: F, t22767: F, t2282: F, t2291: F, t2313: F, t2321: F, t3083: F, t3116: F, t3136: F, t6263: F, t6279: F, t6283: F, t6291: F, t6294: F, t8099: F, t8102: F, t8115: F, t872: F, t882: F, t890: F, t891: F) -> F {
    let t22772 = F::cast_from(0.17544670867903938621e1_f64) * t2291 * t8099 + F::cast_from(0.5848223622634646207e0_f64) * t882 * t22309 * t890 + F::cast_from(0.17544670867903938621e1_f64) * t22740 * t891 + F::cast_from(0.17544670867903938621e1_f64) * t8102 * t2313 + F::cast_from(0.51947577317044391276e2_f64) * t22745 * t2321 + F::cast_from(0.5848223622634646207e0_f64) * t3116 * t6279 + F::cast_from(0.10254018858216406658e4_f64) * t22750 * t6283 + F::cast_from(0.5848223622634646207e0_f64) * t18860 * t1209 + F::cast_from(0.17544670867903938621e1_f64) * t6294 * t3136 + F::new(3.0) * t22757 * t872 + F::new(3.0) * t8115 * t2274 + F::cast_from(0.96491876992155210402e2_f64) * t22762 * t2282 + F::new(1.0) * t3083 * t6263 + F::cast_from(0.2069040516770936012e4_f64) * t22767 * t6291 + F::new(1.0) * t18799 * t1197;
    t22772
}
