//! MGGA_C_PKZB lxc pol — lxc_pol part 11 (v4rho4_3) CSE chunk 943/1340 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_pkzb_lxc_pol_part11_v4rho4_3_chunk943(t10242: f64, t10245: f64, t10253: f64, t10258: f64, t10263: f64, t10267: f64, t10272: f64, t10276: f64, t10278: f64, t2380: f64, t3225: f64, t3235: f64, t3238: f64, t6468: f64, t8319: f64, t8386: f64, t8389: f64, t8394: f64, t8398: f64, t8408: f64, t8458: f64, t8469: f64, t8472: f64) -> f64 {
    let t10280 = -t8386 - 0.47637797908966374413e-4_f64 * t6468 + t8389 - t8394 + 0.28582678745379824648e-3_f64 * t10242 - 0.42874018118069736972e-3_f64 * t2380 * t10245 + 0.45732285992607719436e-2_f64 * t8319 * t3225 + 0.19055119163586549765e-3_f64 * t8398 + 0.12862205435420921092e-2_f64 * t3235 * t10253 - 0.13719685797782315831e-1_f64 * t10258 * t3238 - 0.51448821741683684368e-2_f64 * t3235 * t10263 + 0.25724410870841842184e-2_f64 * t3235 * t10267 + 0.85748036236139473947e-3_f64 * t10272 - 0.28582678745379824648e-3_f64 * t10276 + 0.30488190661738479624e-2_f64 * t10278 + t8408 - t8458 + t8469 - t8472;
    t10280
}
