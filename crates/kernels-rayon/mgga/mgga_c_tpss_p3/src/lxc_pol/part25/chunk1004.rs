//! MGGA_C_TPSS lxc pol — lxc_pol part 25 (v4rho3sigma_7) CSE chunk 1004/1383 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpss_lxc_pol_part25_v4rho3sigma_7_chunk1004(t1246: f64, t5366: f64, t1206: f64, t1228: f64, t13671: f64, t1226: f64, t1229: f64, t13821: f64, t13827: f64, t13835: f64, t13838: f64, t1634: f64, t1636: f64, t4445: f64, t4451: f64, t4453: f64, t4456: f64, t516: f64, t518: f64, t5397: f64, t5401: f64, t5404: f64) -> f64 {
    let t13843 = t1246 * t5366;
    let t13844 = t13843 * t1206;
    let t13847 = t1228 * t13671;
    let t13850 = -12.0_f64 * t1226 * t5401 + 3.0_f64 * t1226 * t5404 + 3.0_f64 * t1229 * t5397 - t13821 * t518 - 24.0_f64 * t13827 * t4453 + 60.0_f64 * t13835 * t4451 - 24.0_f64 * t13838 * t4451 - 12.0_f64 * t13844 * t4451 + 3.0_f64 * t13847 * t516 + 6.0_f64 * t1634 * t4456 + 6.0_f64 * t1636 * t4445;
    t13850
}
