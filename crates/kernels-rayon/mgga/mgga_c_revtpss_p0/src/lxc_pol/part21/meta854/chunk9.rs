//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 3232/3259 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk3232(t1234: f64, t1269: f64, t12699: f64, t12709: f64, t12723: f64, t1280: f64, t1281: f64, t1285: f64, t1287: f64, t16756: f64, t16763: f64, t16768: f64, t17170: f64, t17178: f64, t17188: f64, t17289: f64, t17829: f64, t17875: f64, t17880: f64, t17949: f64, t17951: f64, t3666: f64, t3746: f64, t3763: f64, t45852: f64, t5478: f64, t5491: f64, t56376: f64, t57536: f64, t59032: f64) -> f64 {
    let t59983 = 0.19756347548806534796e1_f64 * t3746 * t16763 - 0.39512695097613069591e1_f64 * t12709 * t17829 + 0.79025390195226139182e1_f64 * t45852 * t17188 + 0.19756347548806534796e1_f64 * t1285 * t1269 * t17170 * t1287 - 0.39512695097613069591e1_f64 * t17880 * t17178 - 0.19756347548806534796e1_f64 * t5478 * t16756 * t17875 - 0.19756347548806534796e1_f64 * t17289 * t3763 - 0.19756347548806534796e1_f64 * t3666 * t16768 - 0.19756347548806534796e1_f64 * t59032 * t1281 - 0.39512695097613069591e1_f64 * t12723 * t17829 + 0.19756347548806534796e1_f64 * t17949 * t57536 * t17951 + 0.19756347548806534796e1_f64 * t12699 * t5491 - 0.65854491829355115987e0_f64 * t1234 * t1280 * t56376;
    t59983
}
