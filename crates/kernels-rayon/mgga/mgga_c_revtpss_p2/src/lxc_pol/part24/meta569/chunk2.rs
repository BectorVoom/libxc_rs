//! MGGA_C_REVTPSS lxc pol — lxc_pol part 24 (v4rho4_4) CSE chunk 1746/1850 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1746(t1469: f64, t1774: f64, t17643: f64, t5819: f64, t24494: f64, t5192: f64, t68255: f64, t81156: f64, t81158: f64, t89824: f64, t89828: f64, t89832: f64, t89839: f64, t89843: f64, t89847: f64, t89851: f64, t89855: f64) -> (f64, f64, f64, f64) {
    let t90253 = t1469 * t1774;
    let t90262 = t17643 * t5819;
    let t90293 = 0.4155806185363551302e3_f64 * t5192 * t24494;
    let t90305 = 0.61805555555555555555e-1_f64 * t89824 - 0.22249999999999999999e0_f64 * t89828 - 0.27469135802469135803e-1_f64 * t89832 + 0.24722222222222222222e-1_f64 * t81156 - 0.74166666666666666668e-1_f64 * t81158 + 0.24722222222222222222e-1_f64 * t68255 - 0.18541666666666666666e-1_f64 * t89839 - 0.24722222222222222222e-1_f64 * t89843 + 0.33375e0_f64 * t89847 + 0.55625000000000000001e-1_f64 * t89851 + 0.74166666666666666668e-1_f64 * t89855;
    (t90253, t90262, t90293, t90305)
}
