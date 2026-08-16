//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 53 (v4rho2sigma2_9) CSE chunk 915/1059 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part53_v4rho2sigma2_9_chunk915(t33825: f64, t33852: f64, t533: f64, t1390: f64, t2075: f64, t7801: f64, t2039: f64, t7890: f64, t1442: f64, t1459: f64, t1983: f64, t2040: f64, t27188: f64, t32235: f64, t32674: f64, t32676: f64, t32679: f64, t33234: f64, t33790: f64, t33793: f64, t4028: f64, t652: f64, t7042: f64, t7458: f64, t7685: f64, t7796: f64, t7802: f64, t7806: f64, t7943: f64, t8607: f64, t8721: f64, t8774: f64, t8805: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t33853 = t33825 + t33852;
    let t33854 = t533 * t33853;
    let t33855 = t33854 * t1390;
    let t33857 = t2075 * t7801;
    let t33874 = t7890 * t2039;
    let t33877 = -t1442 * t8774 - 2.0_f64 * t1459 * t32235 - 3.0_f64 * t1983 * t33790 - t1983 * t33793 + t1983 * t33855 - 4.0_f64 * t2040 * t27188 - 4.0_f64 * t2040 * t33234 - 4.0_f64 * t33857 * t652 - 4.0_f64 * t33874 * t652 - 4.0_f64 * t4028 * t8721 - 4.0_f64 * t7042 * t7796 - 4.0_f64 * t7042 * t7802 - 4.0_f64 * t7042 * t7806 - 4.0_f64 * t7458 * t8721 + t7685 * t8805 - 2.0_f64 * t7943 * t8607 - t32674 - t32676 - t32679;
    (t33853, t33854, t33855, t33857, t33874, t33877)
}
