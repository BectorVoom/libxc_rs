//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 26 (v4rho3sigma_2) CSE chunk 709/1384 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part26_v4rho3sigma_2_chunk709(t6614: f64, t831: f64, t1899: f64, t838: f64, t234: f64, t59: f64, t240: f64, t812: f64) -> (f64, f64, f64, f64, f64) {
    let t6615 = t6614 * t831;
    let t6617 = t1899 * t838;
    let t6618 = 7.0_f64 / 2304.0_f64 * t6617;
    let t6619 = t234 * t59;
    let t6620 = t6619 * t240;
    let t6621 = t812 * t6620;
    (t6615, t6618, t6619, t6620, t6621)
}
