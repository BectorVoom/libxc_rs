//! MGGA_C_PKZB lxc pol — lxc_pol part 9 (v4rho4_1) CSE chunk 1302/1336 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part9_v4rho4_1_chunk1302<F: Float>(t2320: F, t8098: F, t2198: F, t2240: F, t3070: F, t6331: F, t8219: F, t6137: F, t8145: F, t1197: F, t1209: F, t2257: F, t2258: F, t2273: F, t2279: F, t2296: F, t2318: F, t3103: F, t3106: F, t6122: F, t6262: F, t6275: F, t6308: F, t6313: F, t6323: F, t6334: F, t6338: F, t8071: F, t8099: F, t8107: F, t8138: F, t8142: F, t889: F) -> (F, F, F, F) {
    let t22868 = t8098 * t2320;
    let t22878 = F::new(18.0) * t2240 * t3070 * t2198;
    let t22892 = F::new(0.48245938496077605201e2) * t8219 * t6331;
    let t22894 = F::new(18.0) * t6137 * t8145;
    let t22898 = F::new(0.96491876992155210402e2) * t6308 * t8142 - F::new(6.0) * t2257 * t3103 * t2273 - F::new(0.57895126195293126242e3) * t6313 * t8138 * t2258 - F::new(0.35089341735807877242e1) * t2296 * t8099 * t889 + F::new(0.51947577317044391277e2) * t2318 * t22868 * t889 - F::new(0.35089341735807877242e1) * t8071 * t6334 + F::new(0.51947577317044391277e2) * t8107 * t6338 - t22878 - F::new(0.14035736694323150897e2) * t6323 * t1209 * t6122 - F::new(2.0) * t2257 * t1197 * t6262 + F::new(0.96491876992155210402e2) * t2279 * t8138 * t2273 + F::new(0.32163958997385070134e2) * t2279 * t3106 * t6262 - t22892 - t22894 - F::new(24.0) * t6313 * t1197 * t6275;
    (t22878, t22892, t22894, t22898)
}
