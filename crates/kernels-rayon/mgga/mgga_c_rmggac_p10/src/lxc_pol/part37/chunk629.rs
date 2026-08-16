//! MGGA_C_RMGGAC lxc pol — lxc_pol part 37 (v4rho2sigma2_10) CSE chunk 629/1128 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part37_v4rho2sigma2_10_chunk629(t15887: f64, t82: f64, t72: f64, t15206: f64, t15212: f64, t15224: f64, t15865: f64, t884: f64, t1356: f64, t15872: f64, t15273: f64, t14953: f64, t530: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t15888 = t82 * t15887;
    let t15889 = t72 * t15888;
    let t15892 = 0.87596530464506835932e-6_f64 * t15206;
    let t15893 = 0.87596530464506835932e-6_f64 * t15212;
    let t15894 = 0.17519306092901367187e-6_f64 * t15224;
    let t15899 = t884 * t15865;
    let t15900 = 0.59871208509319042821e-1_f64 * t15899;
    let t15901 = t1356 * t15872;
    let t15902 = 0.39914139006212695214e-1_f64 * t15901;
    let t15903 = 0.31062809106223861414e-2_f64 * t15273;
    let t15904 = t530 * t14953;
    (t15888, t15889, t15892, t15893, t15894, t15900, t15902, t15903, t15904)
}
