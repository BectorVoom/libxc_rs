//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 3223/3317 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3223<F: Float>(t1234: F, t1248: F, t12709: F, t12723: F, t12756: F, t1280: F, t1285: F, t1287: F, t1794: F, t1818: F, t20800: F, t21342: F, t21557: F, t24986: F, t24989: F, t3670: F, t3783: F, t5412: F, t5478: F, t5494: F, t59241: F, t59864: F, t59865: F, t6564: F, t6622: F, t6714: F, t70209: F, t82207: F, t82321: F, t82886: F, t84175: F) -> F {
    let t84778 = F::cast_from(0.19756347548806534796e1_f64) * t12756 * t82321 * t3783 - F::cast_from(0.65854491829355115987e0_f64) * t1234 * t1280 * t84175 - F::cast_from(0.19756347548806534796e1_f64) * t12709 * t24986 - F::cast_from(0.19756347548806534796e1_f64) * t12723 * t24986 + F::cast_from(0.39512695097613069591e1_f64) * t3670 * t1280 * t82207 + F::cast_from(0.19756347548806534796e1_f64) * t1285 * t5412 * t6622 * t1287 + F::cast_from(0.39512695097613069591e1_f64) * t59241 * t6714 - F::cast_from(0.19756347548806534796e1_f64) * t12709 * t24989 + F::cast_from(0.19756347548806534796e1_f64) * t6564 * t5494 - F::cast_from(0.19756347548806534796e1_f64) * t5478 * t20800 * t21557 - F::cast_from(0.19756347548806534796e1_f64) * t70209 * t1818 + F::cast_from(0.19756347548806534796e1_f64) * t1285 * t21342 * t1794 * t1287 + F::cast_from(0.15805078039045227836e2_f64) * t59864 * t82886 * t59865 * t1248;
    t84778
}
