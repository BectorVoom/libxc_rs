//! GGA_C_GAPC lxc pol — lxc_pol part 37 (v4rho2sigma2_16) CSE chunk 1138/1445 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gapc_lxc_pol_part37_v4rho2sigma2_16_chunk1138(t11748: f64, t2594: f64, t2600: f64, t11804: f64, t11814: f64, t2599: f64, t11325: f64, t3402: f64, t9934: f64, t11872: f64, t9723: f64, t10072: f64, t11930: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t33195 = t11748 * t2594;
    let t33197 = t11748 * t2600;
    let t33200 = t11814 * t11804 * t2599;
    let t33202 = t3402 * t11325;
    let t33203 = t33202 * t9934;
    let t33205 = t11872 * t9723;
    let t33209 = t11930 * t10072;
    (t33195, t33197, t33200, t33202, t33203, t33205, t33209)
}
