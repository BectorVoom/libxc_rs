//! MGGA_C_REVTPSS lxc pol — lxc_pol part 35 (v4rho3sigma_10) CSE chunk 1005/1234 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part35_v4rho3sigma_10_chunk1005<F: Float>(t24236: F, t5312: F, t13046: F, t24544: F, t1042: F, t13053: F, t1803: F, t6601: F, t1222: F, t1235: F, t1261: F, t12853: F, t13042: F, t13052: F, t1797: F, t21053: F, t21088: F, t21091: F, t21102: F, t24636: F, t24640: F, t24644: F, t24649: F, t24652: F, t3711: F, t484: F) -> F {
    let t24655 = t5312 * t24236;
    let t24663 = t24544 * t13046;
    let t24664 = t1042 * t24663;
    let t24667 = t24544 * t13053;
    let t24668 = t1042 * t24667;
    let t24671 = t6601 * t1803;
    let t24674 = -F::cast_from(0.21437009059034868486e-3_f64) * t1235 * t24636 - F::cast_from(0.7145669686344956162e-3_f64) * t3711 * t24640 + F::cast_from(0.71456696863449561621e-3_f64) * t1261 * t24644 + F::cast_from(0.42874018118069736972e-3_f64) * t3711 * t24649 - t1222 * t24652 / F::new(48.0) + t1222 * t24655 / F::new(72.0) + t12853 - F::cast_from(0.85748036236139473944e-3_f64) * t21053 + F::cast_from(0.45732285992607719436e-2_f64) * t21088 - F::cast_from(0.57165357490759649295e-3_f64) * t21091 + F::cast_from(0.21722835846488666732e-1_f64) * t21102 * t1797 + F::cast_from(0.12862205435420921092e-2_f64) * t13042 * t24664 - F::cast_from(0.12862205435420921092e-2_f64) * t13052 * t24668 - F::cast_from(0.34299214494455789577e-2_f64) * t24671 * t484;
    t24674
}
