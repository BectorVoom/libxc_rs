//! MGGA_C_RMGGAC lxc pol — lxc_pol part 34 (v4rho2sigma2_7) CSE chunk 836/1097 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part34_v4rho2sigma2_7_chunk836(t1986: f64, t3141: f64, t9189: f64, t1602: f64, t793: f64, t14374: f64, t15318: f64, t14363: f64, t15322: f64, t118: f64, t128: f64, t1392: f64) -> (f64, f64, f64, f64, f64) {
    let t75016 = t3141 * t1986 * t9189;
    let t75020 = t3141 * t1986 * t793 * t1602;
    let t75022 = t14374 * t15318;
    let t75024 = t14363 * t15322;
    let t75027 = t118 * t128 * t1392;
    (t75016, t75020, t75022, t75024, t75027)
}
