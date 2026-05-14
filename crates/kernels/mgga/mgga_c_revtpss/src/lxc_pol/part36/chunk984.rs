//! MGGA_C_REVTPSS lxc pol — lxc_pol part 36 (v4rho3sigma_11) CSE chunk 984/1228 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part36_v4rho3sigma_11_chunk984<F: Float>(t1261: F, t13062: F, t17377: F, t17529: F, t17569: F, t17572: F, t1808: F, t20784: F, t20787: F, t20789: F, t21143: F, t21272: F, t24535: F, t24546: F, t24551: F, t464: F, t5274: F, t5391: F, t6619: F, t6625: F, t6631: F, t6635: F, t6673: F) -> (F,) {
    let t24562 = -0.14481890564325777821e-1 * t21272 * t1808 - 0.3811023832717309953e-2 * t5391 * t6673 - 0.63517063878621832552e-3 * t1261 * t24535 - 0.42874018118069736972e-3 * t21143 * t1808 + 0.57165357490759649295e-3 * t20784 - 0.42874018118069736972e-3 * t20787 - 0.45732285992607719436e-2 * t20789 + 0.21437009059034868486e-3 * t13062 * t24546 + 0.85748036236139473944e-3 * t17569 * t6619 - 77.0 / 162.0 * t24551 * t464 + 0.34299214494455789577e-2 * t17529 * t6635 + 0.64311027177104605458e-3 * t5274 * t6625 + 0.12862205435420921092e-2 * t17572 * t6631 - 0.64311027177104605458e-3 * t17377 * t6635;
    (t24562,)
}
