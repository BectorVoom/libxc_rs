//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 55 (v4rho2sigma2_11) CSE chunk 812/1304 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part55_v4rho2sigma2_11_chunk812(t1877: f64, t25: f64, t8366: f64, t8370: f64, t202: f64, t8365: f64, t8369: f64, t193: f64, t2752: f64, t870: f64, t28: f64, t8319: f64, t88: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t8374 = t1877 * t8366 * t25 / 2.0_f64 - t1877 * t8370 * t25 / 2.0_f64;
    let t8418 = t202 * t8365;
    let t8421 = t202 * t8369;
    let t8424 = -t193 * t2752 * t8421 + t193 * t8418 * t870;
    let t8434 = t1877 * t8366 * t28 / 2.0_f64 - t1877 * t8370 * t28 / 2.0_f64;
    let t8444 = 2.0_f64 * t88 * t8319;
    (t8374, t8418, t8421, t8424, t8434, t8444)
}
