//! MGGA_C_RMGGAC lxc pol — lxc_pol part 14 (v4rho3sigma_5) CSE chunk 973/1089 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part14_v4rho3sigma_5_chunk973(t5055: f64, t7444: f64, t236: f64, t321: f64, t3351: f64, t35312: f64, t9211: f64, t2329: f64, t36669: f64, t1970: f64, t1971: f64, t209: f64, t40444: f64, t511: f64) -> (f64, f64, f64, f64) {
    let t40630 = t5055 * t7444;
    let t40637 = t3351 * t35312 * t236 * t9211 * t321;
    let t40647 = t36669 * t2329;
    let t40652 = t1970 * t1971 * t511 * t40444 * t209;
    (t40630, t40637, t40647, t40652)
}
