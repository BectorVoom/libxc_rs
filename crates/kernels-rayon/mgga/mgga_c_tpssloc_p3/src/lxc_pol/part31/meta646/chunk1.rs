//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 31 (v4rho3sigma_7) CSE chunk 1919/2041 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1919(t28342: f64, t81979: f64, t17022: f64, t1880: f64, t1894: f64, t214: f64, t252: f64, t5527: f64, t25038: f64, t6646: f64, t829: f64, t28333: f64, t6562: f64, t794: f64) -> (f64, f64, f64, f64, f64) {
    let t98330 = t81979 * t28342;
    let t98334 = t1880 * t214 * t1894 * t17022;
    let t98336 = t252 * t5527;
    let t98339 = t25038 * t6646 * t98336 * t829;
    let t98342 = t6562 * t794 * t28333;
    (t98330, t98334, t98336, t98339, t98342)
}
