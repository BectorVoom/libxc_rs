//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 3211/3317 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3211<F: Float>(t1209: F, t24864: F, t1210: F, t1211: F, t1215: F, t1274: F, t1277: F, t1294: F, t1775: F, t18054: F, t18059: F, t18062: F, t20697: F, t20700: F, t20722: F, t20753: F, t20760: F, t21382: F, t24519: F, t24524: F, t25022: F, t3556: F, t3567: F, t3572: F, t45438: F, t45552: F, t5225: F, t5237: F, t5251: F, t5497: F, t5498: F, t6580: F, t6587: F, t6588: F, t6703: F, t72877: F, t82514: F, t83551: F, t83567: F) -> F {
    let t84315 = t1209 * t24864;
    let t84337 = -F::cast_from(0.19756347548806534796e1_f64) * t20753 * t5498 + F::cast_from(0.19756347548806534796e1_f64) * t20697 * t5237 + F::cast_from(0.39512695097613069591e1_f64) * t18062 * t6580 + F::cast_from(0.13170898365871023197e1_f64) * t3567 * t1211 * t83551 + F::cast_from(0.19756347548806534796e1_f64) * t1210 * t1277 * t6587 * t5497 - F::cast_from(0.19756347548806534796e1_f64) * t20700 * t5498 + F::cast_from(0.39512695097613069591e1_f64) * t5251 * t21382 + F::cast_from(0.39512695097613069592e1_f64) * t5225 * t20760 + F::cast_from(0.15805078039045227836e2_f64) * t45438 * t1211 * t82514 - F::cast_from(0.65854491829355115987e0_f64) * t3572 * t25022 - F::cast_from(0.65854491829355115987e0_f64) * t84315 * t1215 + F::cast_from(0.39512695097613069591e1_f64) * t3567 * t1211 * t83567 - F::cast_from(0.65854491829355115987e0_f64) * t3556 * t25022 + F::cast_from(0.15805078039045227836e2_f64) * t1274 * t45552 * t24524 * t1294 + F::cast_from(0.79025390195226139182e1_f64) * t18059 * t20722 + F::cast_from(0.39512695097613069591e1_f64) * t18054 * t6703 - F::cast_from(0.19756347548806534796e1_f64) * t18062 * t6588 - F::cast_from(0.39512695097613069591e1_f64) * t3556 * t24519 - F::cast_from(0.19756347548806534796e1_f64) * t72877 * t1775;
    t84337
}
