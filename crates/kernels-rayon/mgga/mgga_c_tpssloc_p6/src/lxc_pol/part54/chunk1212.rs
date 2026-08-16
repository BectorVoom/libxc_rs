//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 54 (v4rho2sigma2_10) CSE chunk 1212/1484 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part54_v4rho2sigma2_10_chunk1212(t1874: f64, t33234: f64, t7042: f64, t7461: f64, t7685: f64, t8641: f64, t26193: f64, t8621: f64, t1985: f64, t225: f64, t567: f64, t7918: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t33236 = 2.0_f64 * t33234 * t1874;
    let t33238 = 2.0_f64 * t7042 * t7461;
    let t33239 = t7685 * t8641;
    let t33240 = t26193 * t8621;
    let t33241 = t1985 * t33240;
    let t33245 = t7918 * t225 * t567;
    (t33236, t33238, t33239, t33240, t33241, t33245)
}
