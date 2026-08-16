//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 50 (v4rho2sigma2_6) CSE chunk 1149/1294 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part50_v4rho2sigma2_6_chunk1149(t30874: f64, t6680: f64, t362: f64, t6768: f64, t82632: f64, t8381: f64, t82573: f64, t23384: f64, t30858: f64, t1920: f64, t30800: f64, t968: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t113576 = t6680 * t30874;
    let t113578 = t362 * t6768;
    let t113600 = 0.36554090374405031922e-2_f64 * t82632 * t8381;
    let t113601 = t82573 * t8381;
    let t113608 = t23384 * t30858;
    let t113611 = t1920 * t968 * t30800;
    (t113576, t113578, t113600, t113601, t113608, t113611)
}
