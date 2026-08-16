//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 45 (v4rho2sigma2_1) CSE chunk 962/1056 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part45_v4rho2sigma2_1_chunk962(t114299: f64, t114172: f64, t22892: f64, t6891: f64, t1307: f64, t6995: f64, t22573: f64, t8449: f64, t31236: f64, t31238: f64, t8326: f64, t9348: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t114300 = 0.16449340668482264365e-1_f64 * t114299;
    let t114316 = t22892 * t114172 * t6891;
    let t114317 = 0.3289868133696452873e-1_f64 * t114316;
    let t114335 = t1307 * t6995;
    let t114360 = t8449 * t22573;
    let t114387 = 4.0_f64 * t31236;
    let t114388 = 4.0_f64 * t31238;
    let t114405 = 2.0_f64 * t9348 * t8326;
    (t114300, t114317, t114335, t114360, t114387, t114388, t114405)
}
