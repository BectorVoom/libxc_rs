//! MGGA_C_REVTPSS lxc pol — lxc_pol part 55 (v4rho2sigma2_10) CSE chunk 1004/1151 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part55_v4rho2sigma2_10_chunk1004<F: Float>(t233: F, t240: F, t27: F, t119833: F, t124: F, t257: F, t10779: F, t775: F, t2684: F, t8486: F, t125: F, t2769: F, t243: F, t9794: F, t25304: F, t8464: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t119835 = t233 * t27 * t240;
    let t119836 = t119833 * t119835;
    let t119837 = t124 * t257;
    let t119839 = t10779 * t119837 * t775;
    let t119840 = t119836 * t119839;
    let t119841 = 0.26773803678175077508e-3 * t119840;
    let t119842 = t8486 * t2684;
    let t119852 = t125 * t2769;
    let t119867 = t243 * t257;
    let t119868 = t9794 * t119867;
    let t119869 = t25304 * t8464 * t119868;
    (t119835, t119836, t119837, t119839, t119841, t119842, t119852, t119867, t119868, t119869)
}
