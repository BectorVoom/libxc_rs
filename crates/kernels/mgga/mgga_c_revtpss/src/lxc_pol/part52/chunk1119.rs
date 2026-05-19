//! MGGA_C_REVTPSS lxc pol — lxc_pol part 52 (v4rho2sigma2_7) CSE chunk 1119/1292 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part52_v4rho2sigma2_7_chunk1119<F: Float>(t119833: F, t119835: F, t124: F, t257: F, t10779: F, t775: F, t2684: F, t8486: F, t125: F, t2769: F, t243: F, t9794: F) -> (F, F, F, F, F, F, F, F) {
    let t119836 = t119833 * t119835;
    let t119837 = t124 * t257;
    let t119839 = t10779 * t119837 * t775;
    let t119840 = t119836 * t119839;
    let t119841 = F::cast_from(0.26773803678175077508e-3_f64) * t119840;
    let t119842 = t8486 * t2684;
    let t119852 = t125 * t2769;
    let t119867 = t243 * t257;
    let t119868 = t9794 * t119867;
    (t119836, t119837, t119839, t119841, t119842, t119852, t119867, t119868)
}
