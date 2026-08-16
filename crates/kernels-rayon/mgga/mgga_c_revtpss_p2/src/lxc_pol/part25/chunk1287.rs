//! MGGA_C_REVTPSS lxc pol — lxc_pol part 25 (v4rho3sigma_0) CSE chunk 1287/1360 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part25_v4rho3sigma_0_chunk1287(t25610: f64, t27639: f64, t1976: f64, t3043: f64, t25604: f64, t7156: f64, t1000: f64, t1097: f64, t11247: f64, t12032: f64, t12079: f64, t12168: f64, t19579: f64, t1982: f64, t1986: f64, t25464: f64, t25591: f64, t25607: f64, t25620: f64, t25662: f64, t25672: f64, t25678: f64, t3060: f64, t3075: f64, t3325: f64, t7145: f64, t7146: f64, t7159: f64, t7161: f64, t93471: f64, t93490: f64, t93867: f64, t93870: f64, t93881: f64, t93884: f64, t93890: f64, t93892: f64, t93893: f64, t989: f64, t999: f64) -> f64 {
    let t93897 = t25610 * t27639;
    let t93901 = t3043 * t1976;
    let t93904 = t7156 * t25604;
    let t93907 = 0.19756347548806534796e1_f64 * t989 * t25662 + 0.52041769129231196772e1_f64 * t25591 * t7145 * t25620 * t999 + 0.52041769129231196772e1_f64 * t25591 * t7145 * t7146 * t3075 - 0.78062653693846795158e1_f64 * t7159 * t25464 * t7161 * t3325 - 0.19756347548806534796e1_f64 * t93867 * t1097 - 0.26020884564615598386e1_f64 * t93471 * t93870 * t11247 * t12168 + 0.26020884564615598386e1_f64 * t93471 * t25672 * t11247 * t12079 + 0.13010442282307799193e1_f64 * t93490 * t25678 - 0.19756347548806534796e1_f64 * t93881 * t1000 + 0.39512695097613069591e1_f64 * t93884 * t3060 - 0.4336814094102599731e0_f64 * t1982 * t12032 * t1986 + 0.26020884564615598386e1_f64 * t93890 * t93892 * t93893 - 0.26020884564615598386e1_f64 * t93897 * t93892 * t19579 - 0.19756347548806534796e1_f64 * t93901 * t1097 + 0.52041769129231196772e1_f64 * t93904 * t25607;
    t93907
}
