//! GGA_C_GAPLOC lxc pol — lxc_pol part 21 (v4rho2sigma2_4) CSE chunk 762/1466 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part21_v4rho2sigma2_4_chunk762(t1865: f64, t2667: f64, t7226: f64, t2717: f64, t702: f64, t1836: f64, t954: f64, t2060: f64, t937: f64, t2532: f64, t779: f64, t1710: f64, t2581: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t7227 = t2667 * t1865;
    let t7228 = t7226 * t7227;
    let t7233 = t2717 * t702;
    let t7236 = t954 * t1836;
    let t7239 = t2060 * t937;
    let t7242 = t779 * t2532;
    let t7245 = t2581 * t1710;
    (t7227, t7228, t7233, t7236, t7239, t7242, t7245)
}
