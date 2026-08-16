//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 1942/3259 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1942(t1390: f64, t13921: f64, t828: f64, t1398: f64, t1882: f64, t3938: f64, t13789: f64, t13869: f64, t13874: f64, t1388: f64, t13880: f64, t1410: f64, t3934: f64, t9753: f64, t9762: f64, t9766: f64, t9771: f64, t9776: f64, t9780: f64, t9786: f64, t9791: f64) -> (f64, f64, f64, f64, f64) {
    let t13923 = t1390 * t828 * t13921;
    let t13926 = t1882 * t1398;
    let t13927 = t13926 * t3938;
    let t13928 = t13789 * t13927;
    let t13931 = -0.20007875121765877254e-1_f64 * t9753 - 0.50820002809285328224e-4_f64 * t9762 + 0.10841600599314203354e-2_f64 * t9766 + 0.71456696863449561619e-5_f64 * t9771 - 0.15244095330869239812e-3_f64 * t9776 - 0.45351183609335988442e-1_f64 * t9780 + 0.85748036236139473944e-2_f64 * t1410 * t13869 + 0.42874018118069736972e-2_f64 * t1410 * t13874 + t13880 - 0.21437009059034868486e-3_f64 * t1388 * t13923 + 0.17149607247227894789e-2_f64 * t3934 * t13928 - t9786 - t9791;
    (t13923, t13926, t13927, t13928, t13931)
}
