//! GGA_C_FT97 lxc pol — lxc_pol part 15 (v4rho4_4) CSE chunk 898/1222 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part15_v4rho4_4_chunk898(t1113: f64, t2427: f64, t1103: f64, t2247: f64, t228: f64, t231: f64, t1123: f64, t2248: f64, t701: f64, t1132: f64, t2999: f64, t89: f64) -> (f64, f64, f64, f64) {
    let t52563 = t2427 * t1113;
    let t52668 = t1103 * t2247;
    let t52670 = t228 * t52668 * t231;
    let t52752 = t701 * t2248 * t1123;
    let t52916 = t89 * t2999 * t1132;
    (t52563, t52670, t52752, t52916)
}
