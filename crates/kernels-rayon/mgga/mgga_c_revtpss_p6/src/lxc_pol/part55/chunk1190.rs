//! MGGA_C_REVTPSS lxc pol — lxc_pol part 55 (v4rho2sigma2_10) CSE chunk 1190/1306 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part55_v4rho2sigma2_10_chunk1190(t126163: f64, t32469: f64, t32474: f64, t119767: f64, t1544: f64, t247: f64, t257: f64, t837: f64, t120046: f64, t33721: f64, t8486: f64, t119875: f64, t33682: f64) -> (f64, f64, f64, f64, f64) {
    let t126164 = t32469 * t126163;
    let t126166 = t32474 * t126163;
    let t126182 = t119767 * t247 * t257 * t1544 * t837;
    let t126185 = t8486 * t120046 * t33721;
    let t126210 = t119875 * t33682;
    (t126164, t126166, t126182, t126185, t126210)
}
