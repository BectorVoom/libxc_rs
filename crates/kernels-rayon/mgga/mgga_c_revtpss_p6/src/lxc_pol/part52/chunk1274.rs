//! MGGA_C_REVTPSS lxc pol — lxc_pol part 52 (v4rho2sigma2_7) CSE chunk 1274/1292 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part52_v4rho2sigma2_7_chunk1274(t102070: f64, t1448: f64, t28196: f64, t34297: f64, t25082: f64, t27153: f64, t37318: f64, t32738: f64, t98450: f64, t4246: f64, t8686: f64, t1502: f64, t32575: f64) -> (f64, f64, f64, f64, f64) {
    let t128917 = 6.0_f64 * t28196 * t102070 * t34297 * t1448;
    let t128920 = 3.0_f64 * t25082 * t37318 * t27153;
    let t128930 = 3.0_f64 * t98450 * t32738;
    let t128932 = t4246 * t8686;
    let t128933 = t1502 * t32575;
    (t128917, t128920, t128930, t128932, t128933)
}
