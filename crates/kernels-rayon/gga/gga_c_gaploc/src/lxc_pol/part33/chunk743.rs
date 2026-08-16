//! GGA_C_GAPLOC lxc pol — lxc_pol part 33 (v4rho2sigma2_16) CSE chunk 743/1464 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part33_v4rho2sigma2_16_chunk743(t2365: f64, t4325: f64, t7025: f64, t1415: f64, t1420: f64, t2317: f64, t900: f64) -> (f64, f64, f64, f64) {
    let t7026 = t2365 * t4325;
    let t7027 = t7025 * t7026;
    let t7029 = t1415 * t1420;
    let t7030 = t900 * t2317;
    (t7026, t7027, t7029, t7030)
}
