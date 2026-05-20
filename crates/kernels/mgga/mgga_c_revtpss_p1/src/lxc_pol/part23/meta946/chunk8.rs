//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 3124/3317 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3124<F: Float>(t1811: F, t20849: F, t1774: F, t21082: F, t6564: F, t1204: F, t1210: F, t1211: F, t1215: F, t1274: F, t1277: F, t1295: F, t17973: F, t17974: F, t17995: F, t18005: F, t18059: F, t20703: F, t20704: F, t20709: F, t20714: F, t20744: F, t20756: F, t21617: F, t21624: F, t24519: F, t24525: F, t24866: F, t24900: F, t34934: F, t3561: F, t3567: F, t3572: F, t3737: F, t5220: F, t5497: F, t5498: F, t56327: F, t56588: F, t6573: F, t6574: F, t6744: F, t6745: F, t72805: F) -> (F, F) {
    let t82204 = t20849 * t1811;
    let t82207 = t1774 * t21082;
    let t82217 = t6564 * t1811;
    let t82220 = -F::cast_from(0.19756347548806534796e1_f64) * t5220 * t21624 - F::cast_from(0.39512695097613069592e1_f64) * t17973 * t17974 * t20709 - F::cast_from(0.11853808529283920877e2_f64) * t56327 * t34934 * t20703 - F::cast_from(0.39512695097613069591e1_f64) * t3561 * t24525 - F::cast_from(0.39512695097613069591e1_f64) * t18059 * t20714 + F::cast_from(0.39512695097613069591e1_f64) * t1274 * t3737 * t5497 * t6744 - F::cast_from(0.39512695097613069591e1_f64) * t3567 * t1277 * t6573 * t5497 + F::cast_from(0.39512695097613069592e1_f64) * t17995 * t20704 - F::cast_from(0.39512695097613069591e1_f64) * t20756 * t5498 + F::cast_from(0.19756347548806534796e1_f64) * t1210 * t1277 * t1774 * t21617 - F::cast_from(0.39512695097613069591e1_f64) * t3572 * t24519 + F::cast_from(0.19756347548806534796e1_f64) * t3572 * t24900 - F::cast_from(0.79025390195226139182e1_f64) * t72805 * t20744 - F::cast_from(0.19756347548806534796e1_f64) * t82204 * t1215 + F::cast_from(0.39512695097613069591e1_f64) * t3567 * t1211 * t82207 - F::cast_from(0.19756347548806534796e1_f64) * t18005 * t6745 + F::cast_from(0.39512695097613069591e1_f64) * t56588 * t6574 + F::cast_from(0.65854491829355115987e0_f64) * t1204 * t24866 - F::cast_from(0.19756347548806534796e1_f64) * t82217 * t1295;
    (t82207, t82220)
}
