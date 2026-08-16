//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 29 (v4rho3sigma_5) CSE chunk 1869/2357 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1869(t4300: f64, t6571: f64, t6553: f64, t1880: f64, t1902: f64, t4142: f64, t1492: f64, t6624: f64, t1519: f64, t214: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t25216 = t6571 * t4300;
    let t25217 = t6553 * t25216;
    let t25218 = t1880 * t25217;
    let t25220 = t4142 * t1902;
    let t25222 = t1492 * t6624;
    let t25224 = t214 * t1519;
    (t25216, t25217, t25218, t25220, t25222, t25224)
}
