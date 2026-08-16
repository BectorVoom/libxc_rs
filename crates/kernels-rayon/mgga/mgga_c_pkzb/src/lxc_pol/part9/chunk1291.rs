//! MGGA_C_PKZB lxc pol — lxc_pol part 9 (v4rho4_1) CSE chunk 1291/1336 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_pkzb_lxc_pol_part9_v4rho4_1_chunk1291(t3135: f64, t6233: f64, t1208: f64, t1209: f64, t18513: f64, t18866: f64, t18875: f64, t18878: f64, t22542: f64, t22544: f64, t2258: f64, t2279: f64, t2296: f64, t2297: f64, t2312: f64, t2318: f64, t3103: f64, t3106: f64, t3136: f64, t3139: f64, t6122: f64, t6224: f64, t6275: f64, t6282: f64, t6288: f64, t6300: f64, t6308: f64, t6323: f64, t8170: f64, t8174: f64, t8177: f64, t8178: f64, t8181: f64) -> f64 {
    let t22662 = t3135 * t6233;
    let t22681 = -t22542 + 0.51947577317044391277e2_f64 * t6300 * t8174 + 0.30762056574649219973e4_f64 * t18875 * t8178 - 0.35089341735807877242e1_f64 * t2296 * t3136 * t2312 - 0.31168546390226634765e3_f64 * t6323 * t8170 * t2297 - 0.11696447245269292414e1_f64 * t2296 * t1209 * t6224 - 0.12304822629859687989e5_f64 * t18866 * t8177 * t6122 + 0.51947577317044391277e2_f64 * t2318 * t8170 * t2312 + 0.30762056574649219973e4_f64 * t6282 * t22662 * t2297 + 0.17315859105681463759e2_f64 * t2318 * t3139 * t6224 + 0.91082604192152556044e5_f64 * t18878 * t1208 * t18513 * t6122 + 18.0_f64 * t6308 * t8181 + 18.0_f64 * t2279 * t3103 * t2258 + 0.11579025239058625248e4_f64 * t6288 * t3106 * t6275 - t22544;
    t22681
}
