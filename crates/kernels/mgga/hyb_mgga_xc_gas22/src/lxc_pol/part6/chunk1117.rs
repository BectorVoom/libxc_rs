//! HYB_MGGA_XC_GAS22 lxc pol — lxc_pol part 6 (v4rho4_2) CSE chunk 1117/1345 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI, M_SQRT2};
use libxc_kernel_math::erf::{erf_approx};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn hyb_mgga_xc_gas22_lxc_pol_part6_v4rho4_2_chunk1117<F: Float>(t2687: F, t700: F, t1096: F, t1110: F, t21841: F, t2647: F, t2727: F, t2730: F, t21837: F, t441: F, t21864: F, t21866: F, t21869: F, t21872: F, t21875: F, t21880: F, t21882: F, t21884: F, t21886: F, t21889: F) -> (F, F, F, F) {
    let t21911 = 1.0 / t2687 / t700;
    let t21932 = 0.35089341735807877242e1 * t1110 * t2647 * t21841 * t1096;
    let t21933 = t2727 * t2727;
    let t21936 = t2730 * t2730;
    let t21940 = 0.24955700379505800916e5 * t441 / t21933 * t21837 / t21936;
    let t21951 = -0.28769444444444444444e1 * t21864 + 0.27618666666666666667e2 * t21866 - 0.10229135802469135803e2 * t21869 + 0.89504938271604938273e1 * t21872 + 0.31310740740740740741e1 * t21875 + 0.366775e-1 * t21880 - 0.58684e0 * t21882 + 0.65204444444444444445e0 * t21884 + 0.5705388888888888889e0 * t21886 + 0.13490888888888888889e1 * t21889;
    (t21911, t21932, t21940, t21951)
}
