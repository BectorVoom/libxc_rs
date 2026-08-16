//! MGGA_C_PKZB lxc pol — lxc_pol part 9 (v4rho4_1) CSE chunk 647/1336 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_pkzb_lxc_pol_part9_v4rho4_1_chunk647(t2916: f64, t301: f64, t761: f64, t758: f64, t2037: f64, t2898: f64) -> (f64, f64, f64) {
    let t2918 = t301 * t2916 * t761;
    let t2919 = t758 * t2918;
    let t2922 = t2037 * t2898;
    (t2918, t2919, t2922)
}
