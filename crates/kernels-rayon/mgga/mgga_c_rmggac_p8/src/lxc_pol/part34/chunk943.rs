//! MGGA_C_RMGGAC lxc pol — lxc_pol part 34 (v4rho2sigma2_7) CSE chunk 943/1097 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part34_v4rho2sigma2_7_chunk943(t73938: f64, t73940: f64, t73944: f64, t1970: f64, t1971: f64, t209: f64, t2447: f64, t476: f64, t515: f64, t2211: f64, t40983: f64, t739: f64) -> (f64, f64, f64, f64, f64) {
    let t76779 = 0.20455996240684006296e-1_f64 * t73938;
    let t76780 = 0.40911992481368012592e-1_f64 * t73940;
    let t76781 = 0.20455996240684006296e-1_f64 * t73944;
    let t76786 = t1970 * t1971 * t515 * t2447 * t476 * t209;
    let t76787 = 0.42564599893297839398e-5_f64 * t76786;
    let t76790 = 0.11974241701863808564e0_f64 * t739 * t2211 * t40983;
    (t76779, t76780, t76781, t76787, t76790)
}
