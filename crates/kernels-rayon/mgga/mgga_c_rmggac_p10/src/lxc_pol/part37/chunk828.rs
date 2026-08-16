//! MGGA_C_RMGGAC lxc pol — lxc_pol part 37 (v4rho2sigma2_10) CSE chunk 828/1128 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part37_v4rho2sigma2_10_chunk828(t3351: f64, t3352: f64, t40983: f64, t515: f64, t15262: f64, t16043: f64, t14107: f64, t5058: f64, t14368: f64, t15353: f64, t14155: f64, t56963: f64) -> (f64, f64, f64, f64, f64) {
    let t74913 = t3351 * t3352 * t515 * t40983;
    let t74915 = t16043 * t15262;
    let t74917 = t5058 * t14107;
    let t74919 = t14368 * t15353;
    let t74921 = t56963 * t14155;
    (t74913, t74915, t74917, t74919, t74921)
}
