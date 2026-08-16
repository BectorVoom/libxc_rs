//! MGGA_C_REVTPSS lxc pol — lxc_pol part 25 (v4rho3sigma_0) CSE chunk 686/1360 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part25_v4rho3sigma_0_chunk686(t4086: f64, t555: f64, t786: f64, t1398: f64, t675: f64, t268: f64, t543: f64, t1419: f64, t72: f64, t1432: f64, t686: f64, t1433: f64, t2470: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t4100 = t4086 * t555;
    let t4101 = t786 * t4100;
    let t4102 = t675 * t1398;
    let t4104 = t268 * t4102 * t543;
    let t4105 = t4101 * t4104;
    let t4107 = t1419 * t72;
    let t4109 = t1432 * t4107 * t686;
    let t4113 = 0.13009920719177044025e-1_f64 * t1432 * t1433 * t2470;
    (t4100, t4101, t4102, t4104, t4105, t4107, t4109, t4113)
}
