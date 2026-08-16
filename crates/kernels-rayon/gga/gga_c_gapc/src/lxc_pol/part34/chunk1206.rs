//! GGA_C_GAPC lxc pol — lxc_pol part 34 (v4rho2sigma2_13) CSE chunk 1206/1427 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gapc_lxc_pol_part34_v4rho2sigma2_13_chunk1206(t3388: f64, t33906: f64, t2494: f64, t3757: f64, t33338: f64, t3781: f64, t17819: f64, t2578: f64, t3679: f64, t11837: f64, t612: f64, t761: f64) -> (f64, f64, f64, f64, f64) {
    let t34207 = t33906 * t3388;
    let t34209 = t3757 * t2494;
    let t34211 = t33338 * t3781;
    let t34214 = t2578 * t3679 * t17819;
    let t34217 = t761 * t612 * t11837;
    (t34207, t34209, t34211, t34214, t34217)
}
