//! MGGA_C_PKZB lxc pol — lxc_pol part 9 (v4rho4_1) CSE chunk 1302/1336 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_pkzb_lxc_pol_part9_v4rho4_1_chunk1302(t2320: f64, t8098: f64, t2198: f64, t2240: f64, t3070: f64, t6331: f64, t8219: f64, t6137: f64, t8145: f64, t1197: f64, t1209: f64, t2257: f64, t2258: f64, t2273: f64, t2279: f64, t2296: f64, t2318: f64, t3103: f64, t3106: f64, t6122: f64, t6262: f64, t6275: f64, t6308: f64, t6313: f64, t6323: f64, t6334: f64, t6338: f64, t8071: f64, t8099: f64, t8107: f64, t8138: f64, t8142: f64, t889: f64) -> (f64, f64, f64, f64) {
    let t22868 = t8098 * t2320;
    let t22878 = 18.0_f64 * t2240 * t3070 * t2198;
    let t22892 = 0.48245938496077605201e2_f64 * t8219 * t6331;
    let t22894 = 18.0_f64 * t6137 * t8145;
    let t22898 = 0.96491876992155210402e2_f64 * t6308 * t8142 - 6.0_f64 * t2257 * t3103 * t2273 - 0.57895126195293126242e3_f64 * t6313 * t8138 * t2258 - 0.35089341735807877242e1_f64 * t2296 * t8099 * t889 + 0.51947577317044391277e2_f64 * t2318 * t22868 * t889 - 0.35089341735807877242e1_f64 * t8071 * t6334 + 0.51947577317044391277e2_f64 * t8107 * t6338 - t22878 - 0.14035736694323150897e2_f64 * t6323 * t1209 * t6122 - 2.0_f64 * t2257 * t1197 * t6262 + 0.96491876992155210402e2_f64 * t2279 * t8138 * t2273 + 0.32163958997385070134e2_f64 * t2279 * t3106 * t6262 - t22892 - t22894 - 24.0_f64 * t6313 * t1197 * t6275;
    (t22878, t22892, t22894, t22898)
}
