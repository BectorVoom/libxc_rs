//! MGGA_C_REVTPSS lxc pol — lxc_pol part 52 (v4rho2sigma2_7) CSE chunk 1002/1144 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part52_v4rho2sigma2_7_chunk1002<F: Float>(t31809: F, t31845: F, t11007: F, t3140: F, t822: F, t31830: F, t1032: F, t7063: F, t233: F, t240: F, t27: F, t124: F, t257: F, t10779: F, t775: F, t2684: F, t8486: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
    let t119818 = t31809 * t31845;
    let t119821 = t3140 * t11007;
    let t119822 = t119821 * t822;
    let t119823 = t31830 * t119822;
    let t119833 = t7063 * t1032;
    let t119835 = t233 * t27 * t240;
    let t119836 = t119833 * t119835;
    let t119837 = t124 * t257;
    let t119839 = t10779 * t119837 * t775;
    let t119840 = t119836 * t119839;
    let t119841 = 0.26773803678175077508e-3 * t119840;
    let t119842 = t8486 * t2684;
    (t119818, t119821, t119822, t119823, t119833, t119835, t119836, t119837, t119839, t119841, t119842)
}
