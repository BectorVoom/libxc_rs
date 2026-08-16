//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 38 (v4rho3tau_2) CSE chunk 802/1335 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part38_v4rho3tau_2_chunk802(t1573: f64, t942: f64, t1581: f64, t950: f64, t2766: f64, t2824: f64, t2912: f64, t2919: f64, t4335: f64, t4340: f64, t4345: f64, t4349: f64, t4363: f64, t4371: f64, t4379: f64, t4381: f64, t4384: f64, t4387: f64, t4390: f64, t4393: f64) -> (f64, f64, f64) {
    let t4449 = t1573 * t942;
    let t4454 = t1581 * t950;
    let t4471 = -0.1294625e1_f64 * t4363 + 0.258925e1_f64 * t4371 + t2912 + 0.10064166666666666667e0_f64 * t2766 + 0.10064166666666666667e0_f64 * t4335 - 0.20128333333333333333e0_f64 * t4340 + 0.60385e0_f64 * t4345 - 0.301925e0_f64 * t4349 + 0.82524375e-1_f64 * t4379 + 0.16504875e0_f64 * t4381 + t2919 + 0.5519e-1_f64 * t2824 + 0.5519e-1_f64 * t4384 - 0.27595e-1_f64 * t4387 + 0.16557e0_f64 * t4390 - 0.82785e-1_f64 * t4393;
    (t4449, t4454, t4471)
}
