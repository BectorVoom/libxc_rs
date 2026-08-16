//! GGA_C_GAPLOC lxc pol — lxc_pol part 30 (v4rho2sigma2_13) CSE chunk 1092/1436 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part30_v4rho2sigma2_13_chunk1092(t28286: f64, t7419: f64, t9805: f64, t22315: f64, t9890: f64, t3294: f64, t739: f64, t7803: f64, t7805: f64, t7383: f64, t948: f64, t9796: f64) -> (f64, f64, f64, f64) {
    let t28289 = 0.10352590007558602413e2_f64 * t9805 * t28286 * t7419;
    let t28290 = t22315 * t9890;
    let t28291 = 0.76685851907841499352e0_f64 * t28290;
    let t28296 = t7803 * t739 * t3294 * t7805;
    let t28297 = 0.1533717038156829987e1_f64 * t28296;
    let t28307 = t9796 * t948 * t7383;
    (t28289, t28291, t28297, t28307)
}
