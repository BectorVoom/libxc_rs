//! HYB_MGGA_XC_GAS22 lxc pol — lxc_pol part 6 (v4rho4_2) CSE chunk 938/1345 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI, M_SQRT2};
use libxc_kernel_math::erf::{erf_approx};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn hyb_mgga_xc_gas22_lxc_pol_part6_v4rho4_2_chunk938<F: Float>(t2307: F, t3435: F, t1370: F, t6640: F, t2315: F, t2292: F, t3443: F, t2322: F, t2330: F, t260: F, t3430: F, t3445: F, t856: F, t8736: F, t8738: F, t8741: F, t8743: F, t8754: F, t8780: F, t8813: F, t8828: F, t8856: F, t8867: F, t8868: F, t8905: F, t8908: F, t8910: F, t8925: F, t8926: F) -> (F, F, F, F) {
    let t8934 = t3435 * t2307;
    let t8937 = t6640 * t1370;
    let t8938 = t8937 * t2315;
    let t8941 = t3443 * t2292;
    let t8944 = -t8736 + t8738 - t8741 + 0.23392894490538584828e1 * t856 * t8743 - 0.34631718211362927518e2 * t2322 * t3445 + t260 * (t8780 + t8813 + t8868 + t8926) - 0.5848223622634646207e0 * t3430 * t2330 + 0.19751673498613801407e-1 * t260 * t8754 + t8828 + t8856 + t8867 + t8905 + t8908 + t8910 - t8925 + 0.11696447245269292414e1 * t856 * t8934 + 0.10389515463408878255e3 * t856 * t8938 - 0.35089341735807877242e1 * t856 * t8941;
    (t8934, t8938, t8941, t8944)
}
