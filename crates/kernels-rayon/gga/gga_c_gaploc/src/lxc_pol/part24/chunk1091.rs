//! GGA_C_GAPLOC lxc pol — lxc_pol part 24 (v4rho2sigma2_7) CSE chunk 1091/1439 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part24_v4rho2sigma2_7_chunk1091(t1986: f64, t9787: f64, t1991: f64, t9797: f64, t21783: f64, t5641: f64, t883: f64, t9805: f64, t3308: f64, t6021: f64, t165: f64, t5397: f64, t935: f64) -> (f64, f64, f64, f64, f64) {
    let t28080 = t1986 * t9787;
    let t28081 = 0.1022478025437886658e1_f64 * t28080;
    let t28084 = t1991 * t9797;
    let t28085 = 0.2044956050875773316e1_f64 * t28084;
    let t28089 = 0.11502877786176224903e1_f64 * t9805 * t5641 * t883 * t21783;
    let t28099 = t6021 * t3308;
    let t28126 = t165 * t935 * t5397;
    (t28081, t28085, t28089, t28099, t28126)
}
