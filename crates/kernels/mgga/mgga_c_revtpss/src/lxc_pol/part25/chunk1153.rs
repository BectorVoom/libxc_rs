//! MGGA_C_REVTPSS lxc pol — lxc_pol part 25 (v4rho3sigma_0) CSE chunk 1153/1212 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part25_v4rho3sigma_0_chunk1153<F: Float>(t1000: F, t1097: F, t11247: F, t12032: F, t12079: F, t12168: F, t19579: F, t1982: F, t1986: F, t25464: F, t25591: F, t25607: F, t25620: F, t25662: F, t25672: F, t25678: F, t3060: F, t3075: F, t3325: F, t7145: F, t7146: F, t7159: F, t7161: F, t93471: F, t93490: F, t93867: F, t93870: F, t93881: F, t93884: F, t93890: F, t93892: F, t93893: F, t93897: F, t93901: F, t93904: F, t989: F, t999: F) -> (F,) {
    let t93907 = 0.19756347548806534796e1 * t989 * t25662 + 0.52041769129231196772e1 * t25591 * t7145 * t25620 * t999 + 0.52041769129231196772e1 * t25591 * t7145 * t7146 * t3075 - 0.78062653693846795158e1 * t7159 * t25464 * t7161 * t3325 - 0.19756347548806534796e1 * t93867 * t1097 - 0.26020884564615598386e1 * t93471 * t93870 * t11247 * t12168 + 0.26020884564615598386e1 * t93471 * t25672 * t11247 * t12079 + 0.13010442282307799193e1 * t93490 * t25678 - 0.19756347548806534796e1 * t93881 * t1000 + 0.39512695097613069591e1 * t93884 * t3060 - 0.4336814094102599731e0 * t1982 * t12032 * t1986 + 0.26020884564615598386e1 * t93890 * t93892 * t93893 - 0.26020884564615598386e1 * t93897 * t93892 * t19579 - 0.19756347548806534796e1 * t93901 * t1097 + 0.52041769129231196772e1 * t93904 * t25607;
    (t93907,)
}
