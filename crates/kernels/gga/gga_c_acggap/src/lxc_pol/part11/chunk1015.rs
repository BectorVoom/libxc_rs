//! GGA_C_ACGGAP lxc pol — lxc_pol part 11 (v4rho3sigma_3) CSE chunk 1015/1066 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part11_v4rho3sigma_3_chunk1015<F: Float>(t35747: F, t2288: F, t3196: F, t13287: F, t31195: F, t2001: F, t4724: F, t1429: F, t7605: F, t1165: F, t20590: F, t604: F, t7337: F, t31593: F, t31544: F, t31565: F, t31570: F, t31585: F, t35731: F, t35733: F, t35737: F, t35738: F, t35740: F, t35742: F, t35744: F) -> (F, F) {
    let t35748 = 0.42874018118069736972e-3 * t35747;
    let t35749 = t2288 * t3196;
    let t35751 = t31195 * t13287 * t35749;
    let t35753 = t2001 * t4724;
    let t35755 = t7605 * t1429;
    let t35756 = 0.17149607247227894789e-1 * t35755;
    let t35759 = t7337 * t1165 * t604 * t20590;
    let t35764 = 0.42874018118069736972e-3 * t31593;
    let t35765 = 0.34299214494455789578e-2 * t35731 - 0.85748036236139473944e-3 * t35733 + 0.66040993808168719343e-1 * t31544 - t35737 + 0.34299214494455789578e-2 * t35738 + 0.80031500487063509014e-2 * t35740 - 0.34299214494455789578e-2 * t35742 - 0.12862205435420921092e-2 * t35744 - t35748 - 0.21437009059034868486e-2 * t35751 - 0.68598428988911579156e-2 * t35753 + t35756 - 0.7862023072401038017e-3 * t35759 + 0.31448092289604152068e-3 * t31565 + 0.62896184579208304136e-3 * t31570 + 0.10718504529517434243e-3 * t31585 - t35764;
    (t35749, t35765)
}
