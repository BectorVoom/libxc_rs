//! MGGA_C_REVTPSS lxc pol — lxc_pol part 51 (v4rho2sigma2_6) CSE chunk 505/1200 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part51_v4rho2sigma2_6_chunk505(t268: f64, t4102: f64, t543: f64, t4101: f64, t1419: f64, t72: f64, t1432: f64, t686: f64, t1433: f64, t2470: f64, t1385: f64, t198: f64, t531: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t4104 = t268 * t4102 * t543;
    let t4105 = t4101 * t4104;
    let t4107 = t1419 * t72;
    let t4109 = t1432 * t4107 * t686;
    let t4113 = 0.13009920719177044025e-1_f64 * t1432 * t1433 * t2470;
    let t4118 = t1385 * t1419;
    let t4139 = t198 * t531;
    (t4104, t4105, t4109, t4113, t4118, t4139)
}
