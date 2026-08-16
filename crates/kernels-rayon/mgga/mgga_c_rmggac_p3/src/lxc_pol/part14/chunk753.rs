//! MGGA_C_RMGGAC lxc pol — lxc_pol part 14 (v4rho3sigma_5) CSE chunk 753/1089 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part14_v4rho3sigma_5_chunk753(t2064: f64, t3928: f64, t798: f64, t1550: f64, t4048: f64, t7778: f64, t2084: f64, t27: f64, t7273: f64, t839: f64, t118: f64, t1986: f64, t209: f64, t35192: f64) -> (f64, f64, f64, f64) {
    let t35407 = t3928 * t2064 * t798;
    let t35413 = t1550 * t7778 * t4048;
    let t35424 = t7273 * t27 * t2084 * t839;
    let t35455 = t1986 * t118 * t35192 * t209;
    (t35407, t35413, t35424, t35455)
}
