//! MGGA_C_RMGGAC lxc pol — lxc_pol part 34 (v4rho2sigma2_7) CSE chunk 765/1097 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part34_v4rho2sigma2_7_chunk765(t14125: f64, t69009: f64, t73825: f64, t14123: f64, t3116: f64, t3128: f64, t68575: f64, t8518: f64, t3351: f64, t3352: f64, t41091: f64, t515: f64) -> (f64, f64, f64) {
    let t73827 = t69009 * t14125 * t73825;
    let t73833 = t3128 * t68575 * t3116 * t14123 * t14125 * t8518;
    let t73837 = t3351 * t3352 * t515 * t41091;
    (t73827, t73833, t73837)
}
