//! MGGA_C_REVTPSS lxc pol — lxc_pol part 20 (v4rho4_0) CSE chunk 836/1798 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk836(t1429: f64, t2435: f64, t1428: f64, t2777: f64, t2439: f64, t1385: f64, t225: f64) -> (f64, f64, f64, f64) {
    let t4082 = 0.73171657588172351096e-2_f64 * t2435 * t1429;
    let t4083 = t2777 * t1428;
    let t4085 = 0.65049603595885220126e-3_f64 * t2439 * t4083;
    let t4086 = t225 * t1385;
    (t4082, t4083, t4085, t4086)
}
