//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 3032/3317 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3032(t1087: f64, t1089: f64, t11940: f64, t15670: f64, t19453: f64, t19484: f64, t19512: f64, t19608: f64, t20128: f64, t24031: f64, t24083: f64, t24138: f64, t3291: f64, t43360: f64, t43446: f64, t4857: f64, t4866: f64, t4930: f64, t4964: f64, t4996: f64, t55685: f64, t6299: f64, t6343: f64, t6365: f64, t65181: f64, t66565: f64, t67714: f64, t67927: f64, t79884: f64) -> f64 {
    let t80798 = -0.39512695097613069591e1_f64 * t4857 * t20128 + 0.39512695097613069591e1_f64 * t15670 * t19512 + 0.19756347548806534796e1_f64 * t1087 * t4930 * t6299 * t1089 - 0.39512695097613069591e1_f64 * t11940 * t3291 * t24031 + 0.19756347548806534796e1_f64 * t1087 * t6343 * t4866 * t1089 - 0.39512695097613069592e1_f64 * t19608 * t19484 - 0.39512695097613069591e1_f64 * t43446 * t79884 * t1089 - 0.39512695097613069591e1_f64 * t67927 * t4964 - 0.19756347548806534796e1_f64 * t4996 * t66565 * t24083 - 0.19756347548806534796e1_f64 * t67714 * t4964 - 0.39512695097613069591e1_f64 * t43360 * t24138 + 0.19756347548806534796e1_f64 * t65181 * t19453 - 0.39512695097613069591e1_f64 * t55685 * t6365;
    t80798
}
