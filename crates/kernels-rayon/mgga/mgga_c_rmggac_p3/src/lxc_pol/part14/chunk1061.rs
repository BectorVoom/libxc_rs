//! MGGA_C_RMGGAC lxc pol — lxc_pol part 14 (v4rho3sigma_5) CSE chunk 1061/1089 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part14_v4rho3sigma_5_chunk1061(t41977: f64, t7244: f64, t9165: f64, t27124: f64, t3351: f64, t3352: f64, t515: f64, t2286: f64, t36542: f64, t1979: f64, t1982: f64, t458: f64, t8601: f64) -> (f64, f64, f64, f64, f64) {
    let t41978 = 0.15965655602485078085e0_f64 * t41977;
    let t41979 = t7244 * t9165;
    let t41980 = 0.19863479950205658386e-4_f64 * t41979;
    let t41983 = t3351 * t3352 * t515 * t27124;
    let t41985 = t36542 * t2286;
    let t41989 = t8601 * t458 * t1979 * t1982;
    (t41978, t41980, t41983, t41985, t41989)
}
