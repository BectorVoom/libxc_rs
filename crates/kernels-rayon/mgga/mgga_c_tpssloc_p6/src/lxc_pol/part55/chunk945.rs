//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 55 (v4rho2sigma2_11) CSE chunk 945/1304 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part55_v4rho2sigma2_11_chunk945(t4028: f64, t6535: f64, t19577: f64, t8643: f64, t22574: f64, t7458: f64, t2314: f64, t7461: f64, t4034: f64, t1873: f64, t5107: f64, t652: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t25969 = 2.0_f64 * t4028 * t6535;
    let t25971 = t8643 * t19577;
    let t25973 = 3.0_f64 * t22574 * t25971;
    let t25975 = 2.0_f64 * t7458 * t6535;
    let t25977 = 2.0_f64 * t2314 * t7461;
    let t25979 = 2.0_f64 * t4034 * t7461;
    let t25980 = t5107 * t1873;
    let t25982 = 2.0_f64 * t652 * t25980;
    (t25969, t25971, t25973, t25975, t25977, t25979, t25980, t25982)
}
