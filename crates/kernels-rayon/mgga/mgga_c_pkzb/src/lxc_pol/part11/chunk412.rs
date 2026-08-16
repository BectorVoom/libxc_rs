//! MGGA_C_PKZB lxc pol — lxc_pol part 11 (v4rho4_3) CSE chunk 412/1340 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_pkzb_lxc_pol_part11_v4rho4_3_chunk412(t164: f64, t600: f64, t133: f64, t614: f64, t1540: f64, t66: f64, t168: f64, t167: f64, t180: f64, t1726: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t1734 = t600 * t164;
    let t1746 = t133 * t614;
    let t1764 = 1.0_f64 / t66 / t1540;
    let t1765 = t168 * t1764;
    let t1768 = 0.56688979511669985553e-2_f64 * t167 * t1765 * t180;
    let t1769 = t167 * t1726;
    (t1734, t1746, t1764, t1765, t1768, t1769)
}
