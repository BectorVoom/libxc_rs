//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 3029/3317 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3029<F: Float>(t1043: F, t1089: F, t12127: F, t16544: F, t16552: F, t16553: F, t16560: F, t19450: F, t19503: F, t19580: F, t20146: F, t24141: F, t3287: F, t3318: F, t43453: F, t43524: F, t43528: F, t4866: F, t4977: F, t55569: F, t55570: F, t55593: F, t55594: F, t55732: F, t56049: F, t67714: F, t78496: F, t78812: F, t79180: F, t79275: F, t999: F) -> F {
    let t80691 = F::cast_from(0.11853808529283920877e2_f64) * t16552 * t19450 * t16553 * t4866 + F::cast_from(0.19756347548806534796e1_f64) * t55732 * t19580 + F::cast_from(0.19756347548806534796e1_f64) * t43453 * t24141 + F::cast_from(0.19756347548806534796e1_f64) * t43528 * t24141 + F::cast_from(0.19756347548806534796e1_f64) * t12127 * t79275 * t3318 - F::cast_from(0.19756347548806534796e1_f64) * t67714 * t4977 - F::cast_from(0.39512695097613069591e1_f64) * t16544 * t20146 - F::cast_from(0.39512695097613069591e1_f64) * t56049 * t19503 + F::cast_from(0.15805078039045227836e2_f64) * t55593 * t78812 * t55594 * t1043 - F::cast_from(0.23707617058567841754e2_f64) * t55569 * t78812 * t55570 * t1043 + F::cast_from(0.39512695097613069591e1_f64) * t43524 * t78496 * t16560 * t999 - F::cast_from(0.19756347548806534796e1_f64) * t3287 * t79180 * t1089;
    t80691
}
