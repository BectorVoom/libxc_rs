//! MGGA_C_REVTPSS lxc pol — lxc_pol part 56 (v4rho2sigma2_11) CSE chunk 1046/1050 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part56_v4rho2sigma2_11_chunk1046<F: Float>(t2167: F, t8249: F, t1913: F, t8978: F, t35034: F, t571: F, t127442: F, t127447: F, t127449: F, t127453: F, t127455: F, t127459: F, t127462: F, t127465: F, t127468: F, t127472: F, t127480: F, t129541: F, t129543: F, t132119: F, t1918: F, t32373: F, t32377: F, t33565: F, t573: F) -> (F, F, F, F) {
    let t132135 = t2167 * t8249;
    let t132141 = t1913 * t8978;
    let t132144 = t571 * t35034;
    let t132152 = t132119 * t573 * param_d + 3.0 * t1918 * t33565 + t127442 + t127447 + t127449 + t127453 + t127455 + t127459 + t127462 + t127465 + t127468 + t127472 + t127480 + 6.0 * t129541 + 12.0 * t129543 + t32373 + t32377;
    (t132135, t132141, t132144, t132152)
}
