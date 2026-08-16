//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 60 (v4rho2sigma2_16) CSE chunk 1012/1064 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part60_v4rho2sigma2_16_chunk1012(t101226: f64, t115027: f64, t126177: f64, t128076: f64, t128080: f64, t128086: f64, t1408: f64, t1877: f64, t1914: f64, t22960: f64, t24191: f64, t25: f64, t2522: f64, t25373: f64, t26744: f64, t28252: f64, t28456: f64, t28459: f64, t28462: f64, t31434: f64, t33466: f64, t33486: f64, t5397: f64, t7114: f64, t7475: f64, t8566: f64, t8569: f64) -> f64 {
    let t128093 = t1877 * t115027 * t28456 + 3.0_f64 * t2522 * t8566 * t28252 + t1877 * t33466 * t1408 - t1877 * t26744 * t33486 - t1877 * t7114 * t5397 * t1914 / 2.0_f64 + 3.0_f64 * t2522 * t33466 * t7475 - t1877 * t31434 * t28462 / 2.0_f64 - 3.0_f64 * t24191 * t126177 + t1877 * t8566 * t5397 / 2.0_f64 + t1877 * t128076 * t25 / 2.0_f64 + 6.0_f64 * t24191 * t25373 * t128080 - t1877 * t31434 * t28459 - 3.0_f64 / 2.0_f64 * t24191 * t22960 * t128086 - t1877 * t101226 * t8569 / 2.0_f64;
    t128093
}
