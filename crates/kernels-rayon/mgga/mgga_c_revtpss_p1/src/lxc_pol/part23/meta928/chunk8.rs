//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 3029/3317 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3029(t1043: f64, t1089: f64, t12127: f64, t16544: f64, t16552: f64, t16553: f64, t16560: f64, t19450: f64, t19503: f64, t19580: f64, t20146: f64, t24141: f64, t3287: f64, t3318: f64, t43453: f64, t43524: f64, t43528: f64, t4866: f64, t4977: f64, t55569: f64, t55570: f64, t55593: f64, t55594: f64, t55732: f64, t56049: f64, t67714: f64, t78496: f64, t78812: f64, t79180: f64, t79275: f64, t999: f64) -> f64 {
    let t80691 = 0.11853808529283920877e2_f64 * t16552 * t19450 * t16553 * t4866 + 0.19756347548806534796e1_f64 * t55732 * t19580 + 0.19756347548806534796e1_f64 * t43453 * t24141 + 0.19756347548806534796e1_f64 * t43528 * t24141 + 0.19756347548806534796e1_f64 * t12127 * t79275 * t3318 - 0.19756347548806534796e1_f64 * t67714 * t4977 - 0.39512695097613069591e1_f64 * t16544 * t20146 - 0.39512695097613069591e1_f64 * t56049 * t19503 + 0.15805078039045227836e2_f64 * t55593 * t78812 * t55594 * t1043 - 0.23707617058567841754e2_f64 * t55569 * t78812 * t55570 * t1043 + 0.39512695097613069591e1_f64 * t43524 * t78496 * t16560 * t999 - 0.19756347548806534796e1_f64 * t3287 * t79180 * t1089;
    t80691
}
