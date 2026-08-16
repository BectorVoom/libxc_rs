//! MGGA_C_PKZB lxc pol — lxc_pol part 9 (v4rho4_1) CSE chunk 1290/1336 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part9_v4rho4_1_chunk1290<F: Float>(t3102: F, t6290: F, t1196: F, t6313: F, t18863: F, t18882: F, t18887: F, t18889: F, t22496: F, t22502: F, t22528: F, t22530: F, t22532: F, t22534: F, t22536: F, t22538: F, t22540: F, t2258: F, t6266: F, t6275: F, t6288: F, t6345: F, t8154: F, t8164: F, t8167: F) -> F {
    let t22627 = t3102 * t6290;
    let t22639 = t6313 * t1196;
    let t22642 = F::cast_from(0.6207121550312808036e4_f64) * t18882 * t8154 + F::cast_from(0.6207121550312808036e4_f64) * t6288 * t22627 * t2258 + F::cast_from(0.19964560303604640732e6_f64) * t18887 * t1196 * t18889 * t6275 - F::cast_from(0.35089341735807877242e1_f64) * t6266 * t8164 - F::cast_from(0.31168546390226634765e3_f64) * t18863 * t8167 - F::cast_from(0.57895126195293126243e3_f64) * t22639 * t6345 - t22496 - t22502 + t22528 + t22530 + t22532 + t22534 + t22536 - t22538 - t22540;
    t22642
}
