//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 3304/3938 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3304(t14586: f64, t4423: f64, t10529: f64, t2782: f64, t18725: f64, t2470: f64, t2798: f64, t10542: f64, t18730: f64, t231: f64, t61749: f64, t2797: f64) -> (f64, f64, f64, f64) {
    let t62628 = t14586 * t4423;
    let t62630 = t2782 * t10529 * t62628;
    let t62633 = t2798 * t18725 * t2470;
    let t62635 = t10542 * t18730;
    let t62637 = t61749 * t231;
    let t62639 = t2782 * t2797 * t62637;
    (t62630, t62633, t62635, t62639)
}
