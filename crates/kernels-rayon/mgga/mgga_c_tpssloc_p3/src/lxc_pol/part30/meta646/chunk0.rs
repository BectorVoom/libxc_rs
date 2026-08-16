//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 30 (v4rho3sigma_6) CSE chunk 2059/2341 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk2059(t1409: f64, t1937: f64, t6722: f64, t14501: f64, t23419: f64, t1015: f64, t23472: f64, t25678: f64, t7554: f64, t82632: f64, t225: f64, t25820: f64) -> (f64, f64, f64, f64, f64) {
    let t88692 = 0.16149102437656156342e-2_f64 * t6722 * t1409 * t1937;
    let t88704 = t23419 * t14501 / 1728.0_f64;
    let t88723 = 0.20186378047070195428e-3_f64 * t23472 * t1015 * t25678;
    let t88731 = t82632 * t7554;
    let t88744 = t25820 * t225;
    (t88692, t88704, t88723, t88731, t88744)
}
