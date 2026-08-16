//! MGGA_C_RMGGAC lxc pol — lxc_pol part 34 (v4rho2sigma2_7) CSE chunk 895/1097 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part34_v4rho2sigma2_7_chunk895(t13819: f64, t8343: f64, t13872: f64, t15296: f64, t13876: f64, t13880: f64, t14117: f64, t68906: f64, t74848: f64, t14374: f64, t15235: f64, t14174: f64, t17787: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t76027 = t13819 * t8343;
    let t76029 = t15296 * t13872;
    let t76031 = t15296 * t13876;
    let t76033 = t15296 * t13880;
    let t76036 = t68906 * t14117 * t74848;
    let t76041 = t14374 * t15235;
    let t76043 = t17787 * t14174;
    (t76027, t76029, t76031, t76033, t76036, t76041, t76043)
}
