//! MGGA_C_RMGGAC lxc pol — lxc_pol part 37 (v4rho2sigma2_10) CSE chunk 873/1128 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part37_v4rho2sigma2_10_chunk873(t15302: f64, t56828: f64, t12200: f64, t1587: f64, t2044: f64, t3076: f64, t15306: f64, t36596: f64, t1614: f64, t7273: f64, t1615: f64, t1986: f64, t3141: f64, t797: f64) -> (f64, f64, f64, f64, f64) {
    let t75648 = t56828 * t15302;
    let t75652 = t12200 * t2044 * t3076 * t1587;
    let t75654 = t36596 * t15306;
    let t75658 = t7273 * t2044 * t3076 * t1614;
    let t75662 = t3141 * t1986 * t797 * t1615;
    (t75648, t75652, t75654, t75658, t75662)
}
