//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 24 (v4rho3sigma_0) CSE chunk 1359/1438 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part24_v4rho3sigma_0_chunk1359(t3009: f64, t984: f64, t10309: f64, t10321: f64, t1065: f64, t10913: f64, t2250: f64, t23313: f64, t23327: f64, t23329: f64, t23330: f64, t23336: f64, t23346: f64, t23353: f64, t23365: f64, t23366: f64, t23593: f64, t23728: f64, t25423: f64, t25429: f64, t25430: f64, t25797: f64, t3010: f64, t6687: f64, t6689: f64, t6690: f64, t6691: f64, t6692: f64, t6699: f64, t82342: f64, t82343: f64, t82357: f64, t82380: f64, t82382: f64, t986: f64) -> (f64, f64) {
    let t82385 = t3009 * t984;
    let t82389 = 0.10966227112321509577e-1_f64 * t25429 * t23329 * t25430 * t10913 - 0.82246703342411321826e-2_f64 * t23327 * t23336 * t23728 + 0.16449340668482264365e-1_f64 * t23327 * t23329 * t82342 * t82343 - 0.82246703342411321826e-2_f64 * t23327 * t23329 * t23330 * t2250 * t1065 - 0.16449340668482264365e-1_f64 * t23327 * t23329 * t25423 * t10913 - 0.82246703342411321826e-2_f64 * t23327 * t82357 * t6691 - 0.24674011002723396548e-1_f64 * t6687 * t3010 * t6699 + 0.13159472534785811492e0_f64 * t23346 * t23366 - 0.24674011002723396548e-1_f64 * t6687 * t23365 * t23313 + 0.27415567780803773942e-2_f64 * t6687 * t6689 * t6690 * t10321 - 0.21932454224643019154e-1_f64 * t6687 * t23593 * t6690 * t10309 - 0.24674011002723396548e-1_f64 * t6687 * t986 * t23353 - 0.16449340668482264365e-1_f64 * t82380 + 0.80418998823691070229e-1_f64 * t82382 * t6692 - 0.24674011002723396548e-1_f64 * t6687 * t82385 * t25797;
    (t82385, t82389)
}
