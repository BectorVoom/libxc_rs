//! MGGA_C_RMGGAC lxc pol — lxc_pol part 34 (v4rho2sigma2_7) CSE chunk 793/1097 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part34_v4rho2sigma2_7_chunk793(t13815: f64, t2323: f64, t7553: f64, t2010: f64, t2131: f64, t8342: f64, t2415: f64, t7399: f64, t68421: f64, t68422: f64, t73785: f64, t15384: f64, t34884: f64) -> (f64, f64, f64, f64, f64) {
    let t74272 = t7553 * t13815 * t2323;
    let t74275 = t2010 * t8342 * t2131;
    let t74278 = t2010 * t2415 * t7399;
    let t74281 = t68421 * t68422 * t73785;
    let t74283 = t34884 * t15384;
    (t74272, t74275, t74278, t74281, t74283)
}
