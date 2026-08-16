//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 31 (v4rho3sigma_7) CSE chunk 1948/2041 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1948(t1527: f64, t776: f64, t23270: f64, t25038: f64, t25191: f64, t23204: f64, t28294: f64, t6562: f64, t1493: f64, t254: f64, t28263: f64, t1880: f64, t23237: f64) -> (f64, f64, f64, f64, f64) {
    let t98960 = t1527 * t776;
    let t98963 = t25038 * t23270 * t25191 * t98960;
    let t98966 = t6562 * t23204 * t28294;
    let t98975 = t1493 * t254;
    let t98983 = t6562 * t23204 * t28263;
    let t98986 = t1880 * t23237 * t28263;
    (t98963, t98966, t98975, t98983, t98986)
}
