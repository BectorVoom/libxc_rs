//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 3220/3259 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk3220(t13126: f64, t1770: f64, t1234: f64, t12621: f64, t12690: f64, t12732: f64, t1285: f64, t1287: f64, t13108: f64, t13121: f64, t13130: f64, t13133: f64, t16750: f64, t17821: f64, t17861: f64, t1811: f64, t1825: f64, t3568: f64, t3670: f64, t3755: f64, t3759: f64, t3778: f64, t5245: f64, t5326: f64, t5486: f64, t57200: f64, t57498: f64) -> f64 {
    let t59550 = t1770 * t13126;
    let t59579 = 0.65854491829355115987e0_f64 * t1285 * t1811 * t12732 * t1287 + 0.65854491829355115987e0_f64 * t59550 * t13130 - 0.19756347548806534796e1_f64 * t1234 * t3759 * t16750 - 0.19756347548806534796e1_f64 * t5326 * t13121 - 0.19756347548806534796e1_f64 * t1234 * t13133 * t5245 - 0.65854491829355115987e0_f64 * t1234 * t5486 * t12621 + 0.65854491829355115987e0_f64 * t12690 * t1825 + 0.39512695097613069591e1_f64 * t3670 * t17821 * t3568 + 0.65854491829355115987e0_f64 * t1770 * t13108 + 0.19756347548806534796e1_f64 * t17861 * t3778 - 0.65854491829355115987e0_f64 * t3755 * t57200 * t1287 - 0.19756347548806534796e1_f64 * t3755 * t57498 * t1287;
    t59579
}
