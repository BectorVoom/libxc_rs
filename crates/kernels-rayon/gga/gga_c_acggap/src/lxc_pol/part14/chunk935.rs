//! GGA_C_ACGGAP lxc pol — lxc_pol part 14 (v4rho3sigma_6) CSE chunk 935/1223 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part14_v4rho3sigma_6_chunk935(t3645: f64, t611: f64, t103: f64, t2162: f64, t104: f64, t9081: f64, t694: f64, t9090: f64, t9083: f64, t96: f64, t1662: f64, t1679: f64, t2541: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t32222 = 0.65854491829355115987e0_f64 * t3645 * t611;
    let t32241 = t103 * t2162;
    let t33352 = t104 * t9081;
    let t33357 = 6.0_f64 * t694 * t9090;
    let t33388 = 2.0_f64 * t96 * t9083;
    let t33403 = 2.0_f64 * t1679 * t2541 * t1662;
    (t32222, t32241, t33352, t33357, t33388, t33403)
}
