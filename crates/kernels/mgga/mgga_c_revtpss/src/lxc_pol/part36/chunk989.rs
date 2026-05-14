//! MGGA_C_REVTPSS lxc pol — lxc_pol part 36 (v4rho3sigma_11) CSE chunk 989/1228 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part36_v4rho3sigma_11_chunk989<F: Float>(t1222: F, t1235: F, t1261: F, t12853: F, t13042: F, t13052: F, t1797: F, t21053: F, t21088: F, t21091: F, t21102: F, t24636: F, t24640: F, t24644: F, t24649: F, t24652: F, t24655: F, t24664: F, t24668: F, t24671: F, t3711: F, t484: F) -> (F,) {
    let t24674 = -0.21437009059034868486e-3 * t1235 * t24636 - 0.7145669686344956162e-3 * t3711 * t24640 + 0.71456696863449561621e-3 * t1261 * t24644 + 0.42874018118069736972e-3 * t3711 * t24649 - t1222 * t24652 / 48.0 + t1222 * t24655 / 72.0 + t12853 - 0.85748036236139473944e-3 * t21053 + 0.45732285992607719436e-2 * t21088 - 0.57165357490759649295e-3 * t21091 + 0.21722835846488666732e-1 * t21102 * t1797 + 0.12862205435420921092e-2 * t13042 * t24664 - 0.12862205435420921092e-2 * t13052 * t24668 - 0.34299214494455789577e-2 * t24671 * t484;
    (t24674,)
}
