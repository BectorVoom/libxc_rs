//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 19 (v4rho4_0) CSE chunk 1314/1497 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1314(t10413: f64, t10414: f64, t10422: f64, t10393: f64, t3070: f64, t11046: f64, t42387: f64, t10457: f64, t820: f64, t10409: f64, t10936: f64, t3180: f64) -> (f64, f64, f64, f64, f64) {
    let t42478 = t10413 * t10422 * t10414;
    let t42481 = t3070 * t10422 * t10393;
    let t42483 = t11046 * t42387;
    let t42488 = t820 * t10457;
    let t42490 = t3070 * t42488 * t10409;
    let t42496 = t3180 * t10936;
    (t42478, t42481, t42483, t42490, t42496)
}
