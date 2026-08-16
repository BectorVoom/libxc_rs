//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 55 (v4rho2sigma2_11) CSE chunk 1268/1304 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part55_v4rho2sigma2_11_chunk1268(t7015: f64, t96334: f64, t7769: f64, t85416: f64, t24972: f64, t26550: f64, t1873: f64, t96311: f64, t34385: f64, t580: f64, t117773: f64, t119795: f64, t119796: f64, t119810: f64, t122875: f64, t122897: f64, t122910: f64, t122914: f64, t122918: f64, t122921: f64, t122923: f64, t122925: f64, t1266: f64, t1459: f64, t1774: f64, t27290: f64, t32595: f64, t34229: f64, t34372: f64, t4072: f64, t5107: f64, t650: f64, t652: f64, t7266: f64, t8860: f64, t8913: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t123294 = t96334 * t7015;
    let t123296 = t85416 * t7769;
    let t123298 = t24972 * t26550;
    let t123306 = t96311 * t1873;
    let t125074 = t34385 * t580;
    let t125094 = -2.0_f64 * t4072 * t652 * t8913 - 2.0_f64 * t117773 * t1459 - t1266 * t34229 - t1774 * t32595 - 4.0_f64 * t27290 * t7266 - t34372 * t650 - t5107 * t8860 + t119795 - t119796 - t119810 - 4.0_f64 * t122875 - 4.0_f64 * t122897 + 6.0_f64 * t122910 + 6.0_f64 * t122914 - 4.0_f64 * t122918 - 4.0_f64 * t122921 - 4.0_f64 * t122923 - 2.0_f64 * t122925;
    (t123294, t123296, t123298, t123306, t125074, t125094)
}
