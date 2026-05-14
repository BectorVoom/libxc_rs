//! MGGA_C_REVTPSS lxc pol — lxc_pol part 36 (v4rho3sigma_11) CSE chunk 1081/1228 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part36_v4rho3sigma_11_chunk1081<F: Float>(t343: F, t5842: F, t136: F, t1797: F, t1808: F, t26821: F, t26844: F, t26849: F, t26867: F, t26880: F, t29020: F, t29023: F, t29027: F, t29031: F, t29034: F, t29037: F, t29065: F, t29083: F, t30789: F, t464: F, t484: F, t6619: F, t6625: F, t6631: F, t6635: F, t6640: F, t6679: F, t7618: F, t7624: F) -> (F, F, F) {
    let t30799 = t5842 * t343;
    let t30800 = t30799 * t136;
    let t30805 = -0.45732285992607719436e-2 * t29020 * t1797 + 0.57165357490759649296e-3 * t29023 + 0.57165357490759649296e-3 * t26880 * t6619 + 0.42874018118069736972e-3 * t7618 * t6625 + 0.85748036236139473944e-3 * t26844 * t6631 - 0.42874018118069736972e-3 * t26849 * t6635 - t29027 / 54.0 - t26821 - t29031 / 432.0 - 0.3811023832717309953e-3 * t29034 + 0.42874018118069736972e-3 * t30789 * t484 + 0.30488190661738479624e-2 * t29083 * t1808 - 0.28582678745379824648e-3 * t7624 * t6679 - 0.57165357490759649296e-3 * t29065 - 0.57165357490759649296e-3 * t26867 * t6640 + 11.0 / 108.0 * t30800 * t464 - 0.57165357490759649296e-3 * t29037 * t1808;
    (t30799, t30800, t30805)
}
