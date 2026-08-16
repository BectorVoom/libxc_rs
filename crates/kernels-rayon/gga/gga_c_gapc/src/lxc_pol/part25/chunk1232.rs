//! GGA_C_GAPC lxc pol — lxc_pol part 25 (v4rho2sigma2_4) CSE chunk 1232/1444 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gapc_lxc_pol_part25_v4rho2sigma2_4_chunk1232(t137: f64, t3074: f64, t34509: f64, t5126: f64, t26578: f64, t34503: f64, t203: f64, t27596: f64, t5698: f64, t6: f64, t1030: f64, t144: f64, t33521: f64, t4052: f64) -> (f64, f64, f64, f64) {
    let t34535 = t3074 * t137;
    let t34537 = t34509 * t34535 * t5126;
    let t34539 = t34503 * t26578;
    let t34546 = t5698 * t203 * t6 * t27596;
    let t34547 = t1030 * t4052 * t33521 * t144 * t34546;
    (t34535, t34537, t34539, t34547)
}
