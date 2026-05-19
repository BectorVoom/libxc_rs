//! MGGA_C_REVTPSS lxc pol — lxc_pol part 26 (v4rho3sigma_1) CSE chunk 1020/1225 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part26_v4rho3sigma_1_chunk1020<F: Float>(t1204: F, t1234: F, t12769: F, t1281: F, t1285: F, t12966: F, t12975: F, t12987: F, t13108: F, t13112: F, t13118: F, t13121: F, t13127: F, t13130: F, t13134: F, t13142: F, t13144: F, t13148: F, t13150: F, t13153: F, t13156: F, t13161: F, t3666: F, t3670: F, t3746: F, t3751: F, t3760: F, t3763: F, t3767: F, t3778: F, t3782: F, t3787: F, t460: F) -> F {
    let t13164 = -F::cast_from(0.39512695097613069591e1_f64) * t3666 * t3760 - F::cast_from(0.65854491829355115987e0_f64) * t1234 * t12769 + F::cast_from(0.65854491829355115987e0_f64) * t460 * t13108 + F::cast_from(0.39512695097613069591e1_f64) * t3767 * t13112 + F::cast_from(0.19756347548806534796e1_f64) * t1204 * t3787 + F::cast_from(0.19756347548806534796e1_f64) * t1285 * t13118 - F::cast_from(0.19756347548806534796e1_f64) * t1234 * t13121 + F::cast_from(0.39512695097613069591e1_f64) * t12966 * t3751 + F::cast_from(0.65854491829355115987e0_f64) * t13127 * t13130 - F::cast_from(0.19756347548806534796e1_f64) * t1234 * t13134 - F::cast_from(0.19756347548806534796e1_f64) * t12975 * t1281 - F::cast_from(0.19756347548806534796e1_f64) * t3666 * t3763 - F::cast_from(0.39512695097613069591e1_f64) * t13142 * t13144 + F::cast_from(0.39512695097613069591e1_f64) * t13148 * t13150 - F::cast_from(0.19756347548806534796e1_f64) * t3782 * t13153 + F::cast_from(0.39512695097613069591e1_f64) * t3670 * t13156 + F::cast_from(0.19756347548806534796e1_f64) * t3746 * t3778 - F::cast_from(0.39512695097613069591e1_f64) * t12987 * t13161;
    t13164
}
