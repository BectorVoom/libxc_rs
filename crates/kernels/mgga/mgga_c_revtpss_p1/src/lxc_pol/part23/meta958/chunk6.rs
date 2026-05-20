//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 3219/3317 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3219<F: Float>(t13045: F, t6622: F, t1204: F, t1248: F, t13148: F, t13149: F, t17192: F, t17846: F, t20900: F, t21456: F, t21459: F, t21468: F, t21473: F, t24915: F, t24981: F, t43350: F, t45707: F, t45852: F, t471: F, t5332: F, t5446: F, t5463: F, t5464: F, t59650: F, t59657: F, t59681: F, t59737: F, t6717: F, t72270: F, t72386: F, t82886: F, t84462: F) -> F {
    let t84636 = t13045 * t6622;
    let t84641 = -F::cast_from(0.19756347548806534796e1_f64) * t21456 * t21468 - F::cast_from(0.19756347548806534796e1_f64) * t72270 * t5446 - F::cast_from(0.39512695097613069591e1_f64) * t59657 * t6717 + F::cast_from(0.65854491829355115987e0_f64) * t1204 * t24915 + F::cast_from(0.39512695097613069591e1_f64) * t13148 * t84462 * t13149 - F::cast_from(0.65854491829355115987e0_f64) * t59737 * t82886 * t43350 * t1248 * t471 - F::cast_from(0.19756347548806534796e1_f64) * t17192 * t21459 - F::cast_from(0.39512695097613069591e1_f64) * t72386 * t5446 + F::cast_from(0.39512695097613069591e1_f64) * t45707 * t24981 + F::cast_from(0.39512695097613069591e1_f64) * t45852 * t24981 + F::cast_from(0.19756347548806534796e1_f64) * t59681 * t21473 + F::cast_from(0.39512695097613069591e1_f64) * t5463 * t5332 * t5464 * t20900 + F::cast_from(0.11853808529283920877e2_f64) * t17846 * t59650 * t84636 * t1248;
    t84641
}
