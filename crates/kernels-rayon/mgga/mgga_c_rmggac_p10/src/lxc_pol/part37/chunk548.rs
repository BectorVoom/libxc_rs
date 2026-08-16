//! MGGA_C_RMGGAC lxc pol — lxc_pol part 37 (v4rho2sigma2_10) CSE chunk 548/1128 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part37_v4rho2sigma2_10_chunk548(t14413: f64, t2039: f64, t638: f64, t31: f64, t703: f64, t2046: f64, t2050: f64, t2211: f64, t7799: f64, t739: f64, t7879: f64, t884: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t14415 = t638 * t2039 * t14413;
    let t14417 = t703 * t31;
    let t14419 = t2046 * t2050 * t14417;
    let t14421 = t2211 * t7799;
    let t14422 = t739 * t14421;
    let t14423 = 0.11974241701863808564e0_f64 * t14422;
    let t14424 = t2211 * t7879;
    let t14425 = t884 * t14424;
    (t14415, t14417, t14419, t14421, t14423, t14424, t14425)
}
