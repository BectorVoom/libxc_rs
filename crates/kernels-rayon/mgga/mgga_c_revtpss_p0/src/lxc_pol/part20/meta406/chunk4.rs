//! MGGA_C_REVTPSS lxc pol — lxc_pol part 20 (v4rho4_0) CSE chunk 1505/1798 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1505(t1063: f64, t11160: f64, t247: f64, t3109: f64, t11620: f64, t73: f64, t3153: f64, t12166: f64, t15905: f64, t994: f64, t11631: f64, t999: f64) -> (f64, f64, f64, f64, f64) {
    let t42606 = t1063 * t247 * t3109 * t11160;
    let t42610 = t11620 * t73;
    let t42615 = t11620 * t3153;
    let t42621 = t994 * t12166 * t15905;
    let t42622 = t11631 * t999;
    (t42606, t42610, t42615, t42621, t42622)
}
