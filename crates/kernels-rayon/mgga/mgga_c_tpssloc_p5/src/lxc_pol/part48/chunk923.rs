//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 48 (v4rho2sigma2_4) CSE chunk 923/1034 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part48_v4rho2sigma2_4_chunk923(t3856: f64, t6936: f64, t6943: f64, t3851: f64, t22827: f64, t22828: f64, t22817: f64, t794: f64, t8462: f64, t1369: f64, t31165: f64, t3872: f64, t8466: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t113972 = t6936 * t6943 * t3856;
    let t113975 = t6936 * t6943 * t3851;
    let t113978 = t22827 * t6943 * t22828;
    let t113981 = t22817 * t794 * t8462;
    let t113983 = t31165 * t1369;
    let t113985 = t8466 * t3872;
    (t113972, t113975, t113978, t113981, t113983, t113985)
}
