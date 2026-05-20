//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 3033/3317 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3033<F: Float>(t24042: F, t994: F, t1000: F, t1076: F, t1079: F, t1096: F, t16305: F, t16371: F, t1652: F, t1696: F, t19403: F, t19429: F, t20178: F, t20191: F, t23603: F, t23621: F, t24047: F, t3047: F, t3052: F, t3063: F, t42067: F, t4764: F, t4773: F, t4941: F, t4947: F, t5016: F, t55413: F, t6245: F, t6251: F, t6351: F, t64550: F, t64629: F, t64636: F, t64845: F, t68170: F, t80274: F, t80310: F, t80349: F, t80391: F, t80425: F, t80458: F, t80490: F, t80519: F, t80557: F, t80592: F, t80622: F, t80654: F, t80691: F, t80724: F, t80764: F, t80798: F) -> F {
    let t80810 = t994 * t24042;
    let t80819 = F::cast_from(0.39512695097613069591e1_f64) * t20191 * t4764 - F::cast_from(0.39512695097613069591e1_f64) * t64845 * t1652 + F::cast_from(0.15805078039045227836e2_f64) * t1076 * t42067 * t24047 * t1096 - F::cast_from(0.39512695097613069591e1_f64) * t20191 * t4773 + F::cast_from(0.39512695097613069591e1_f64) * t16371 * t6351 + F::cast_from(0.19756347548806534796e1_f64) * t3063 * t23621 + F::cast_from(0.39512695097613069591e1_f64) * t55413 * t6245 + F::cast_from(0.39512695097613069591e1_f64) * t16305 * t6251 + F::cast_from(0.19756347548806534796e1_f64) * t3047 * t23621 - F::cast_from(0.19756347548806534796e1_f64) * t64629 * t1652 + F::cast_from(0.39512695097613069591e1_f64) * t20191 * t4941 + F::cast_from(0.39512695097613069592e1_f64) * t20178 * t4947 - F::cast_from(0.65854491829355115987e0_f64) * t1076 * t1079 * (t80274 + t80310 + t80349 + t80391 + t80425 + t80458 + t80490 + t80519 + t80557 + t80592 + t80622 + t80654 + t80691 + t80724 + t80764 + t80798) - F::cast_from(0.19756347548806534796e1_f64) * t20178 * t5016 - F::cast_from(0.79025390195226139182e1_f64) * t68170 * t19429 - F::cast_from(0.65854491829355115987e0_f64) * t80810 * t1000 - F::cast_from(0.19756347548806534796e1_f64) * t64636 * t1696 - F::cast_from(0.79025390195226139182e1_f64) * t64550 * t19403 + F::cast_from(0.39512695097613069591e1_f64) * t3052 * t23603;
    t80819
}
