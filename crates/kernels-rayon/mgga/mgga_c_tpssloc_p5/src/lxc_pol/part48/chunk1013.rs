//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 48 (v4rho2sigma2_4) CSE chunk 1013/1034 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part48_v4rho2sigma2_4_chunk1013(t12524: f64, t31817: f64, t66940: f64, t8657: f64, t31814: f64, t2039: f64, t22479: f64, t3941: f64, t7230: f64, t1873: f64, t84078: f64, t94165: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t115980 = 54.0_f64 * t12524 * t31817;
    let t115983 = 54.0_f64 * t66940 * t8657;
    let t115990 = 54.0_f64 * t12524 * t31814;
    let t115995 = 27.0_f64 * t3941 * t2039 * t22479;
    let t116000 = 0.135e2_f64 * t7230 * t22479;
    let t116004 = 0.135e2_f64 * t84078 * t1873;
    let t116006 = 27.0_f64 * t94165 * t1873;
    (t115980, t115983, t115990, t115995, t116000, t116004, t116006)
}
