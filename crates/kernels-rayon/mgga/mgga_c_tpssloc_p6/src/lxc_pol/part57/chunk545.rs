//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 57 (v4rho2sigma2_13) CSE chunk 545/1049 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part57_v4rho2sigma2_13_chunk545(t1012: f64, t6754: f64, t1036: f64, t1942: f64, t1039: f64, t1940: f64, t354: f64, t1946: f64, t225: f64, t1949: f64, t968: f64, t1920: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t6755 = t1012 * t6754;
    let t6763 = t1942 * t1036 / 2304.0_f64;
    let t6764 = t1940 * t1039;
    let t6765 = t354 * t6764;
    let t6771 = t1946 * t225;
    let t6781 = t968 * t1949;
    let t6783 = 0.27415567780803773942e-2_f64 * t1920 * t6781;
    (t6755, t6763, t6764, t6765, t6771, t6783)
}
