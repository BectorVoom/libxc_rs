//! GGA_C_FT97 lxc pol — lxc_pol part 28 (v4rho2sigma2_6) CSE chunk 1067/1189 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part28_v4rho2sigma2_6_chunk1067(t3238: f64, t32457: f64, t103163: f64, t1332: f64, t108: f64, t1286: f64, t1337: f64, t137525: f64, t145585: f64, t25601: f64, t25847: f64, t25863: f64, t28: f64, t32016: f64, t32378: f64, t32387: f64, t32392: f64, t34581: f64, t34784: f64, t369: f64, t438: f64, t5495: f64, t5501: f64, t5748: f64, t6414: f64, t6455: f64, t984: f64) -> (f64, f64, f64) {
    let t145741 = t3238 * t32457;
    let t145761 = t103163 * t1332;
    let t145769 = t5501 * t137525 * t25601 / 9.0_f64 - t32016 * t25863 / 18.0_f64 - 2.0_f64 * t145741 + t1286 * t28 * t25847 * t1337 / 3.0_f64 + t1286 * t28 * t369 * t145585 * t108 / 6.0_f64 + t1286 * t28 * t32378 * t984 / 6.0_f64 - t438 * t34784 + t5495 * t34581 / 6.0_f64 - t6414 * t32392 / 3.0_f64 - 4.0_f64 * t145761 + t1286 * t28 * t6455 * t5748 / 3.0_f64 - 2.0_f64 / 3.0_f64 * t6414 * t32387;
    (t145741, t145761, t145769)
}
