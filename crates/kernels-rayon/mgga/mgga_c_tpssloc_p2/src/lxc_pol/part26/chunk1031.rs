//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 26 (v4rho3sigma_2) CSE chunk 1031/1384 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part26_v4rho3sigma_2_chunk1031(t12049: f64, t12095: f64, t12119: f64, t12144: f64, t225: f64, t1995: f64, t68: f64, t1307: f64, t3734: f64) -> (f64, f64, f64) {
    let t12147 = (t12049 + t12095 + t12119 + t12144) * t225;
    let t12155 = t68 * t1995;
    let t12156 = t3734 * t1307;
    (t12147, t12155, t12156)
}
