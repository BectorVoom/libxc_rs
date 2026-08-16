//! MGGA_C_REVTPSS lxc pol — lxc_pol part 55 (v4rho2sigma2_10) CSE chunk 1082/1306 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part55_v4rho2sigma2_10_chunk1082(t248: f64, t33711: f64, t125: f64, t1579: f64, t246: f64, t244: f64, t31838: f64, t1561: f64, t31846: f64, t4450: f64, t31851: f64, t8486: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t33712 = t33711 * t248;
    let t33714 = t125 * t1579;
    let t33715 = t246 * t33714;
    let t33716 = t244 * t33715;
    let t33717 = t31838 * t33716;
    let t33719 = t31846 * t1561;
    let t33721 = t246 * t4450;
    let t33722 = t31851 * t33721;
    let t33723 = t8486 * t33722;
    (t33712, t33714, t33715, t33716, t33717, t33719, t33721, t33722, t33723)
}
