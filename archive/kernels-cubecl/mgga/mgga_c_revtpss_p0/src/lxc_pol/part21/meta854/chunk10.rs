//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 3233/3259 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk3233<F: Float>(t1204: F, t17852: F, t1209: F, t1284: F, t5412: F, t17845: F, t17306: F, t3754: F, t1234: F, t1248: F, t12719: F, t12741: F, t1287: F, t13112: F, t17178: F, t17345: F, t17633: F, t17821: F, t17849: F, t17856: F, t17864: F, t17883: F, t17934: F, t3552: F, t3584: F, t3755: F, t3756: F, t44421: F, t45666: F, t5436: F, t5443: F, t5477: F, t5481: F, t59187: F) -> F {
    let t59987 = t1204 * t17852;
    let t60008 = t1209 * t1284 * t5412;
    let t60013 = t1204 * t17845;
    let t60019 = t17306 * t3754;
    let t60022 = F::cast_from(0.39512695097613069591e1_f64) * t17934 * t13112 - F::cast_from(0.11853808529283920877e2_f64) * t59987 * t17856 - F::cast_from(0.11853808529283920877e2_f64) * t45666 * t17345 * t1248 * t1287 + F::cast_from(0.19756347548806534796e1_f64) * t5436 * t12741 - F::cast_from(0.39512695097613069591e1_f64) * t17864 * t17178 - F::cast_from(0.19756347548806534796e1_f64) * t3552 * t5477 * t5481 - F::cast_from(0.19756347548806534796e1_f64) * t3755 * t59187 * t1287 - F::cast_from(0.19756347548806534796e1_f64) * t1234 * t17821 * t3584 - F::cast_from(0.39512695097613069591e1_f64) * t60008 * t3756 + F::cast_from(0.39512695097613069591e1_f64) * t44421 * t5443 + F::cast_from(0.11853808529283920877e2_f64) * t60013 * t17849 - F::cast_from(0.19756347548806534796e1_f64) * t3755 * t17633 * t17883 + F::cast_from(0.39512695097613069591e1_f64) * t60019 * t12719;
    t60022
}
