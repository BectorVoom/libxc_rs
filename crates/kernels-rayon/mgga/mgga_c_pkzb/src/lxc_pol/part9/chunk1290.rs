//! MGGA_C_PKZB lxc pol — lxc_pol part 9 (v4rho4_1) CSE chunk 1290/1336 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_pkzb_lxc_pol_part9_v4rho4_1_chunk1290(t3102: f64, t6290: f64, t1196: f64, t6313: f64, t18863: f64, t18882: f64, t18887: f64, t18889: f64, t22496: f64, t22502: f64, t22528: f64, t22530: f64, t22532: f64, t22534: f64, t22536: f64, t22538: f64, t22540: f64, t2258: f64, t6266: f64, t6275: f64, t6288: f64, t6345: f64, t8154: f64, t8164: f64, t8167: f64) -> f64 {
    let t22627 = t3102 * t6290;
    let t22639 = t6313 * t1196;
    let t22642 = 0.6207121550312808036e4_f64 * t18882 * t8154 + 0.6207121550312808036e4_f64 * t6288 * t22627 * t2258 + 0.19964560303604640732e6_f64 * t18887 * t1196 * t18889 * t6275 - 0.35089341735807877242e1_f64 * t6266 * t8164 - 0.31168546390226634765e3_f64 * t18863 * t8167 - 0.57895126195293126243e3_f64 * t22639 * t6345 - t22496 - t22502 + t22528 + t22530 + t22532 + t22534 + t22536 - t22538 - t22540;
    t22642
}
