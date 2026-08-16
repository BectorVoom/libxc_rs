//! MGGA_C_REVTPSS lxc pol — lxc_pol part 20 (v4rho4_0) CSE chunk 1031/1798 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1031(t10665: f64, t10871: f64, t827: f64, t828: f64, t10666: f64, t2648: f64, t2741: f64, t2710: f64, t826: f64, t9732: f64, t234: f64, t2735: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t10872 = t10665 * t10871;
    let t10874 = t827 * t828 * t10872;
    let t10878 = t827 * t828 * t10666;
    let t10881 = t2741 * t2648;
    let t10885 = 0.81322168495418382223e-4_f64 * t2710 * t9732 * t826;
    let t10886 = t2735 * t234;
    (t10872, t10874, t10878, t10881, t10885, t10886)
}
