//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 53 (v4rho2sigma2_9) CSE chunk 1043/1059 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part53_v4rho2sigma2_9_chunk1043(t117014: f64, t121004: f64, t121007: f64, t124018: f64, t124040: f64, t124069: f64, t124093: f64, t124122: f64, t124176: f64, t124205: f64, t124281: f64, t1390: f64, t1459: f64, t1774: f64, t1983: f64, t2040: f64, t26878: f64, t26977: f64, t27145: f64, t27163: f64, t27188: f64, t32263: f64, t32674: f64, t32676: f64, t32679: f64, t33234: f64, t33899: f64, t5107: f64, t533: f64, t7042: f64, t7050: f64, t7057: f64, t7061: f64, t7217: f64, t7796: f64, t8329: f64, t8607: f64, t8711: f64) -> f64 {
    let t124292 = -2.0_f64 * t8607 * t26878 - 4.0_f64 * t26977 * t7796 - 4.0_f64 * t7042 * t27163 - t32674 - t32676 - t32679 - t32263 * t1774 - t8711 * t5107 - 4.0_f64 * t27188 * t7057 - 4.0_f64 * t121004 * t2040 - 4.0_f64 * t121007 * t2040 - 4.0_f64 * t33234 * t7050 + 2.0_f64 * t8607 * t27145 - 2.0_f64 * t1983 * t7217 * t33899 - t8329 + t1983 * t533 * (t124018 + t124040 + t124069 + t124093 + t124122 + t124176 + t124205 + t124281) * t1390 - 4.0_f64 * t27188 * t7061 - 2.0_f64 * t117014 * t1459;
    t124292
}
