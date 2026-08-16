//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 22 (v4rho4_3) CSE chunk 1479/2721 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1479(t14722: f64, t14704: f64, t11147: f64, t1409: f64, t11153: f64, t3242: f64, t3966: f64, t3247: f64, t1667: f64, t2403: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t14723 = 4.0_f64 / 9.0_f64 * t14722;
    let t14724 = 2.0_f64 / 9.0_f64 * t14704;
    let t14725 = t11147 * t1409;
    let t14730 = t11153 * t1409;
    let t14735 = t3242 * t3966;
    let t14748 = t3247 * t3966;
    let t14766 = t2403 * t1667;
    (t14723, t14724, t14725, t14730, t14735, t14748, t14766)
}
