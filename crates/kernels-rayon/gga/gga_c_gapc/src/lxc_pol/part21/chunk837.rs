//! GGA_C_GAPC lxc pol — lxc_pol part 21 (v4rho2sigma2_0) CSE chunk 837/1307 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gapc_lxc_pol_part21_v4rho2sigma2_0_chunk837(t3444: f64, t9750: f64, t315: f64, t9184: f64, t3443: f64, t277: f64, t9179: f64, t2438: f64, t3439: f64, t325: f64, t8769: f64, t2639: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t9751 = t9750 * t3444;
    let t9753 = t9184 * t315;
    let t9754 = t3443 * t9753;
    let t9756 = t277 * t9179;
    let t9757 = t2438 * t3439;
    let t9758 = t9756 * t9757;
    let t9760 = t325 * t8769;
    let t9761 = t9760 * t2639;
    (t9751, t9754, t9756, t9758, t9760, t9761)
}
