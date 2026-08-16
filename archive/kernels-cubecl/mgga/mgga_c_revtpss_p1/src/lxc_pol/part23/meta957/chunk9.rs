//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 3210/3317 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3210<F: Float>(t1210: F, t1211: F, t12628: F, t1274: F, t1277: F, t1294: F, t1775: F, t17973: F, t17974: F, t17986: F, t17987: F, t17995: F, t18037: F, t18059: F, t18114: F, t1829: F, t20704: F, t20709: F, t20714: F, t20727: F, t20740: F, t21348: F, t21389: F, t21390: F, t21621: F, t21624: F, t24509: F, t25015: F, t3561: F, t3737: F, t5245: F, t5251: F, t5414: F, t5417: F, t5423: F, t59464: F, t6564: F, t6574: F, t6580: F, t6588: F, t6744: F, t72787: F, t72794: F, t72959: F, t82525: F) -> F {
    let t84290 = F::cast_from(0.39512695097613069591e1_f64) * t3561 * t24509 - F::cast_from(0.19756347548806534796e1_f64) * t18037 * t6588 - F::cast_from(0.11853808529283920877e2_f64) * t5417 * t21348 - F::cast_from(0.19756347548806534796e1_f64) * t72959 * t1829 - F::cast_from(0.79025390195226139182e1_f64) * t72794 * t21390 + F::cast_from(0.39512695097613069592e1_f64) * t18059 * t20704 + F::cast_from(0.39512695097613069591e1_f64) * t18114 * t6580 - F::cast_from(0.11853808529283920877e2_f64) * t12628 * t1211 * t82525 - F::cast_from(0.39512695097613069591e1_f64) * t17995 * t20714 - F::cast_from(0.39512695097613069591e1_f64) * t17986 * t17987 * t20709 + F::cast_from(0.39512695097613069591e1_f64) * t59464 * t6574 - F::cast_from(0.19756347548806534796e1_f64) * t5251 * t21624 + F::cast_from(0.19756347548806534796e1_f64) * t1210 * t1277 * t5245 * t6744 + F::cast_from(0.79025390195226139182e1_f64) * t17973 * t21389 * t20740 + F::cast_from(0.13170898365871023197e1_f64) * t1274 * t3737 * t25015 * t1294 - F::cast_from(0.39512695097613069591e1_f64) * t72787 * t1775 - F::cast_from(0.39512695097613069591e1_f64) * t17973 * t17974 * t20727 + F::cast_from(0.19756347548806534796e1_f64) * t6564 * t5414 + F::cast_from(0.19756347548806534796e1_f64) * t21621 * t5423;
    t84290
}
