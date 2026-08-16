//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 34 (v4rho3sigma_10) CSE chunk 648/1250 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part34_v4rho3sigma_10_chunk648(t1899: f64, t838: f64, t234: f64, t59: f64, t240: f64, t812: f64) -> (f64, f64, f64, f64) {
    let t6617 = t1899 * t838;
    let t6619 = t234 * t59;
    let t6620 = t6619 * t240;
    let t6621 = t812 * t6620;
    (t6617, t6619, t6620, t6621)
}
