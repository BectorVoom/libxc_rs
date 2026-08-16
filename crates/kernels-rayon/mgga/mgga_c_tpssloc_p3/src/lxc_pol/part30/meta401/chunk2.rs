//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 30 (v4rho3sigma_6) CSE chunk 1528/2341 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1528(t2970: f64, t5828: f64, t973: f64, t16558: f64, t978: f64, t977: f64, t343: f64, t5836: f64, t984: f64, t4546: f64, t10231: f64, t5817: f64) -> (f64, f64, f64, f64) {
    let t17769 = t2970 * t5828;
    let t17770 = t973 * t17769;
    let t17772 = t978 * t16558;
    let t17773 = t977 * t17772;
    let t17777 = t5836 * t984 * t343;
    let t17778 = t4546 * t17777;
    let t17783 = t10231 * t5817;
    (t17770, t17773, t17778, t17783)
}
