//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 3032/3317 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3032<F: Float>(t1087: F, t1089: F, t11940: F, t15670: F, t19453: F, t19484: F, t19512: F, t19608: F, t20128: F, t24031: F, t24083: F, t24138: F, t3291: F, t43360: F, t43446: F, t4857: F, t4866: F, t4930: F, t4964: F, t4996: F, t55685: F, t6299: F, t6343: F, t6365: F, t65181: F, t66565: F, t67714: F, t67927: F, t79884: F) -> F {
    let t80798 = -F::cast_from(0.39512695097613069591e1_f64) * t4857 * t20128 + F::cast_from(0.39512695097613069591e1_f64) * t15670 * t19512 + F::cast_from(0.19756347548806534796e1_f64) * t1087 * t4930 * t6299 * t1089 - F::cast_from(0.39512695097613069591e1_f64) * t11940 * t3291 * t24031 + F::cast_from(0.19756347548806534796e1_f64) * t1087 * t6343 * t4866 * t1089 - F::cast_from(0.39512695097613069592e1_f64) * t19608 * t19484 - F::cast_from(0.39512695097613069591e1_f64) * t43446 * t79884 * t1089 - F::cast_from(0.39512695097613069591e1_f64) * t67927 * t4964 - F::cast_from(0.19756347548806534796e1_f64) * t4996 * t66565 * t24083 - F::cast_from(0.19756347548806534796e1_f64) * t67714 * t4964 - F::cast_from(0.39512695097613069591e1_f64) * t43360 * t24138 + F::cast_from(0.19756347548806534796e1_f64) * t65181 * t19453 - F::cast_from(0.39512695097613069591e1_f64) * t55685 * t6365;
    t80798
}
