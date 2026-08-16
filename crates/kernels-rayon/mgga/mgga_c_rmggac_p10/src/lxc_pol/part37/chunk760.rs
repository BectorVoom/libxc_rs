//! MGGA_C_RMGGAC lxc pol — lxc_pol part 37 (v4rho2sigma2_10) CSE chunk 760/1128 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part37_v4rho2sigma2_10_chunk760(t14117: f64, t68455: f64, t8842: f64, t15208: f64, t68922: f64, t236: f64, t31817: f64, t14125: f64, t68448: f64, t15339: f64, t3119: f64, t34855: f64) -> (f64, f64, f64, f64, f64) {
    let t73779 = t68455 * t14117 * t8842;
    let t73783 = t68922 * t15208;
    let t73785 = t236 * t31817;
    let t73787 = t68448 * t14125 * t73785;
    let t73790 = t15339 * t34855 * t3119;
    (t73779, t73783, t73785, t73787, t73790)
}
