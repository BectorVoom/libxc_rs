//! MGGA_C_RMGGAC lxc pol — lxc_pol part 34 (v4rho2sigma2_7) CSE chunk 548/1097 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part34_v4rho2sigma2_7_chunk548(t13825: f64, t13829: f64, t13833: f64, t13837: f64, t13842: f64, t270: f64, t703: f64, t2039: f64, t638: f64, t31: f64, t2046: f64, t2050: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t14407 = 0.76860658247009135557e-5_f64 * t13825;
    let t14408 = 0.68186654135613354325e-2_f64 * t13829;
    let t14409 = 0.93188427318671584245e-2_f64 * t13833;
    let t14410 = 0.15531404553111930708e-1_f64 * t13837;
    let t14411 = 0.31062809106223861415e-2_f64 * t13842;
    let t14413 = t703 * t270;
    let t14415 = t638 * t2039 * t14413;
    let t14417 = t703 * t31;
    let t14419 = t2046 * t2050 * t14417;
    (t14407, t14408, t14409, t14410, t14411, t14413, t14415, t14417, t14419)
}
