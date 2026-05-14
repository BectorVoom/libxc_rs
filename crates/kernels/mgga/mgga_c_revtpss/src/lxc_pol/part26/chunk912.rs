//! MGGA_C_REVTPSS lxc pol — lxc_pol part 26 (v4rho3sigma_1) CSE chunk 912/1080 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part26_v4rho3sigma_1_chunk912<F: Float>(t12629: F, t1280: F, t1204: F, t1234: F, t12769: F, t1281: F, t1285: F, t12966: F, t12975: F, t12987: F, t13108: F, t13112: F, t13118: F, t13121: F, t13127: F, t13130: F, t13134: F, t13142: F, t13144: F, t13148: F, t13150: F, t13153: F, t13156: F, t3666: F, t3670: F, t3746: F, t3751: F, t3760: F, t3763: F, t3767: F, t3778: F, t3782: F, t3787: F, t460: F) -> (F,) {
    let t13161 = t1280 * t12629;
    let t13164 = -0.39512695097613069591e1 * t3666 * t3760 - 0.65854491829355115987e0 * t1234 * t12769 + 0.65854491829355115987e0 * t460 * t13108 + 0.39512695097613069591e1 * t3767 * t13112 + 0.19756347548806534796e1 * t1204 * t3787 + 0.19756347548806534796e1 * t1285 * t13118 - 0.19756347548806534796e1 * t1234 * t13121 + 0.39512695097613069591e1 * t12966 * t3751 + 0.65854491829355115987e0 * t13127 * t13130 - 0.19756347548806534796e1 * t1234 * t13134 - 0.19756347548806534796e1 * t12975 * t1281 - 0.19756347548806534796e1 * t3666 * t3763 - 0.39512695097613069591e1 * t13142 * t13144 + 0.39512695097613069591e1 * t13148 * t13150 - 0.19756347548806534796e1 * t3782 * t13153 + 0.39512695097613069591e1 * t3670 * t13156 + 0.19756347548806534796e1 * t3746 * t3778 - 0.39512695097613069591e1 * t12987 * t13161;
    (t13164,)
}
