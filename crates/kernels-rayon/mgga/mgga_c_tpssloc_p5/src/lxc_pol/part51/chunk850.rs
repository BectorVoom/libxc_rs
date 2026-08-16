//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 51 (v4rho2sigma2_7) CSE chunk 850/1475 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part51_v4rho2sigma2_7_chunk850(t533: f64, t7939: f64, t1390: f64, t2095: f64, t5161: f64, t113: f64, t1442: f64, t1459: f64, t1774: f64, t1849: f64, t1983: f64, t2036: f64, t2040: f64, t2075: f64, t2079: f64, t2096: f64, t4028: f64, t510: f64, t574: f64, t652: f64, t7042: f64, t7458: f64, t7685: f64, t7787: f64, t7796: f64, t7802: f64, t7806: f64, t7890: f64, t7900: f64, t7904: f64) -> (f64, f64, f64, f64) {
    let t7940 = t533 * t7939;
    let t7941 = t7940 * t1390;
    let t7943 = t2095 * t5161;
    let t7945 = -t113 * t7890 - t1442 * t2075 - 2.0_f64 * t1459 * t7042 - t1774 * t2036 + t1849 * t2079 + 3.0_f64 * t1983 * t7904 + t1983 * t7941 - t1983 * t7943 - 2.0_f64 * t2040 * t4028 - 2.0_f64 * t2040 * t7458 + t2096 * t7685 - t510 * t7787 + t574 * t7900 - 2.0_f64 * t652 * t7796 - 2.0_f64 * t652 * t7802 - 2.0_f64 * t652 * t7806;
    (t7940, t7941, t7943, t7945)
}
