//! MGGA_C_RMGGAC lxc pol — lxc_pol part 34 (v4rho2sigma2_7) CSE chunk 885/1097 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part34_v4rho2sigma2_7_chunk885(t15258: f64, t16156: f64, t21719: f64, t9188: f64, t9197: f64, t15235: f64, t68432: f64, t21713: f64, t68422: f64, t8503: f64, t21714: f64, t8507: f64) -> (f64, f64, f64, f64, f64) {
    let t75820 = t16156 * t15258;
    let t75823 = t21719 * t9188 * t9197;
    let t75825 = t68432 * t15235;
    let t75828 = t21713 * t68422 * t8503;
    let t75831 = t21713 * t21714 * t8507;
    (t75820, t75823, t75825, t75828, t75831)
}
