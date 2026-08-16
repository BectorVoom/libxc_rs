//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 25 (v4rho3sigma_1) CSE chunk 638/1226 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part25_v4rho3sigma_1_chunk638(t641: f64, t71: f64, t107: f64, t625: f64, t63: f64, t656: f64, t666: f64, t25: f64, t776: f64, t154: f64, t781: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t6509 = t71 * t641;
    let t6528 = t625 * t107;
    let t6530 = t63 * t656;
    let t6531 = t6530 * t666;
    let t6542 = t25 * t776;
    let t6546 = t781 * t154;
    (t6509, t6528, t6530, t6531, t6542, t6546)
}
