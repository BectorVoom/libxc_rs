//! GGA_C_ACGGAP lxc pol — lxc_pol part 13 (v4rho3sigma_5) CSE chunk 1148/1213 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part13_v4rho3sigma_5_chunk1148(t31593: f64, t31544: f64, t31565: f64, t31570: f64, t31585: f64, t35731: f64, t35733: f64, t35737: f64, t35738: f64, t35740: f64, t35742: f64, t35744: f64, t35748: f64, t35751: f64, t35753: f64, t35756: f64, t35759: f64) -> f64 {
    let t35764 = 0.42874018118069736972e-3_f64 * t31593;
    let t35765 = 0.34299214494455789578e-2_f64 * t35731 - 0.85748036236139473944e-3_f64 * t35733 + 0.66040993808168719343e-1_f64 * t31544 - t35737 + 0.34299214494455789578e-2_f64 * t35738 + 0.80031500487063509014e-2_f64 * t35740 - 0.34299214494455789578e-2_f64 * t35742 - 0.12862205435420921092e-2_f64 * t35744 - t35748 - 0.21437009059034868486e-2_f64 * t35751 - 0.68598428988911579156e-2_f64 * t35753 + t35756 - 0.7862023072401038017e-3_f64 * t35759 + 0.31448092289604152068e-3_f64 * t31565 + 0.62896184579208304136e-3_f64 * t31570 + 0.10718504529517434243e-3_f64 * t31585 - t35764;
    t35765
}
