//! GGA_C_FT97 kxc pol — kxc_pol part 2 (v3rho3_1) CSE chunk 884/1007 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_kxc_pol_part2_v3rho3_1_chunk884(t2371: f64, t3821: f64, t713: f64, t193: f64, t89: f64, t11401: f64, t665: f64, t3705: f64, t668: f64, t737: f64, t2999: f64, t1132: f64, t1636: f64) -> (f64, f64, f64, f64, f64) {
    let t13725 = t2371 * t3821;
    let t13726 = t13725 * t713;
    let t13728 = t89 * t193 * t13726;
    let t13730 = t11401 * t665;
    let t13732 = t89 * t13730 * t3705;
    let t13734 = t737 * t668;
    let t13736 = t89 * t2999 * t13734;
    let t13739 = t89 * t1636 * t1132;
    (t13728, t13730, t13732, t13736, t13739)
}
