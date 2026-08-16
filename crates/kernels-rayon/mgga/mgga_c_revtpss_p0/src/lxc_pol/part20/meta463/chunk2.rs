//! MGGA_C_REVTPSS lxc pol — lxc_pol part 20 (v4rho4_0) CSE chunk 1762/1798 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1762(t13847: f64, t4057: f64, t9816: f64, t9819: f64, t9807: f64, t9962: f64, t9832: f64, t2482: f64, t27: f64, t9991: f64, t221: f64, t4019: f64, t9995: f64) -> (f64, f64, f64, f64) {
    let t47282 = t9816 * t13847 * t9819 * t4057;
    let t47284 = t9962 * t9807;
    let t47286 = t9962 * t9832;
    let t47293 = t2482 * t9991 * t27;
    let t47296 = t47293 * t4019 * t221 * t9995;
    (t47282, t47284, t47286, t47296)
}
