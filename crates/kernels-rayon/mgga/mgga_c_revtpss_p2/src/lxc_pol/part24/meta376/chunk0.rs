//! MGGA_C_REVTPSS lxc pol — lxc_pol part 24 (v4rho4_4) CSE chunk 1269/1850 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1269(t24236: f64, t5312: f64, t13046: f64, t24544: f64, t1042: f64, t13053: f64, t1803: f64, t6601: f64, t1222: f64, t1235: f64, t1261: f64, t12853: f64, t13042: f64, t13052: f64, t1797: f64, t21053: f64, t21088: f64, t21091: f64, t21102: f64, t24636: f64, t24640: f64, t24644: f64, t24649: f64, t24652: f64, t3711: f64, t484: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t24655 = t5312 * t24236;
    let t24663 = t24544 * t13046;
    let t24664 = t1042 * t24663;
    let t24667 = t24544 * t13053;
    let t24668 = t1042 * t24667;
    let t24671 = t6601 * t1803;
    let t24674 = -0.21437009059034868486e-3_f64 * t1235 * t24636 - 0.7145669686344956162e-3_f64 * t3711 * t24640 + 0.71456696863449561621e-3_f64 * t1261 * t24644 + 0.42874018118069736972e-3_f64 * t3711 * t24649 - t1222 * t24652 / 48.0_f64 + t1222 * t24655 / 72.0_f64 + t12853 - 0.85748036236139473944e-3_f64 * t21053 + 0.45732285992607719436e-2_f64 * t21088 - 0.57165357490759649295e-3_f64 * t21091 + 0.21722835846488666732e-1_f64 * t21102 * t1797 + 0.12862205435420921092e-2_f64 * t13042 * t24664 - 0.12862205435420921092e-2_f64 * t13052 * t24668 - 0.34299214494455789577e-2_f64 * t24671 * t484;
    (t24655, t24663, t24664, t24667, t24668, t24671, t24674)
}
