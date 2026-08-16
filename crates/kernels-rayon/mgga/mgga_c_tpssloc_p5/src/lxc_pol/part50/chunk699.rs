//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 50 (v4rho2sigma2_6) CSE chunk 699/1294 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part50_v4rho2sigma2_6_chunk699(t6689: f64, t6691: f64, t1922: f64, t986: f64, t1049: f64, t225: f64, t387: f64, t345: f64, t340: f64, t344: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t6692 = t6689 * t6691;
    let t6695 = t986 * t1922;
    let t6698 = t1049 * t225;
    let t6699 = t6698 * t387;
    let t6700 = t345 * t6699;
    let t6703 = t340 * t344;
    (t6692, t6695, t6698, t6699, t6700, t6703)
}
