//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 28 (v4rho3sigma_4) CSE chunk 1676/2041 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1676(t1512: f64, t23041: f64, t4166: f64, t6613: f64, t831: f64, t23053: f64, t4236: f64, t6614: f64, t1878: f64, t23033: f64, t221: f64, t4255: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t25144 = t23041 * t1512;
    let t25146 = t4166 * t6613;
    let t25147 = t25146 * t831;
    let t25149 = t23053 * t1512;
    let t25151 = t6614 * t4236;
    let t25154 = t1878 * t23033;
    let t25155 = t221 * t4255;
    (t25144, t25146, t25147, t25149, t25151, t25154, t25155)
}
