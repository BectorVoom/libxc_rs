//! MGGA_C_PKZB lxc pol — lxc_pol part 9 (v4rho4_1) CSE chunk 620/1336 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_pkzb_lxc_pol_part9_v4rho4_1_chunk620(t2710: f64, t2723: f64, t1066: f64, t204: f64, t648: f64, t1837: f64, t972: f64) -> (f64, f64, f64) {
    let t2724 = t2710 + t2723;
    let t2730 = t204 * t648 * t1066;
    let t2732 = t1837 * t972;
    (t2724, t2730, t2732)
}
