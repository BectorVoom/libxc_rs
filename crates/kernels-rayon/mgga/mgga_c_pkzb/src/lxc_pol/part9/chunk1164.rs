//! MGGA_C_PKZB lxc pol — lxc_pol part 9 (v4rho4_1) CSE chunk 1164/1336 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_pkzb_lxc_pol_part9_v4rho4_1_chunk1164(t545: f64, t6798: f64, t83: f64, t16626: f64, t16631: f64, t16701: f64, t16873: f64, t16875: f64, t19776: f64, t19778: f64, t19796: f64, t19798: f64, t19799: f64, t19804: f64, t19806: f64, t19807: f64, t19823: f64, t19825: f64) -> (f64, f64) {
    let t20325 = t83 * t6798 * t545;
    let t20326 = 3.0_f64 * t20325;
    let t20327 = t19776 + t19778 + t16626 - t16631 - t19796 - t19798 - t19799 + t19804 - t19806 + t19807 + t16873 + t16701 - t19823 + t19825 + t20326 - t16875;
    (t20326, t20327)
}
