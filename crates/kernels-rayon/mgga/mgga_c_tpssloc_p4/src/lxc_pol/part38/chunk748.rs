//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 38 (v4rho3tau_2) CSE chunk 748/1335 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part38_v4rho3tau_2_chunk748(t3879: f64, t539: f64, t1373: f64, t225: f64, t1376: f64, t566: f64, t68: f64, t1385: f64, t3787: f64, t562: f64, t3793: f64, t1338: f64, t1372: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t3880 = t539 * t3879;
    let t3882 = t1373 * t225;
    let t3886 = 1.0_f64 / t1376 / t566;
    let t3887 = t68 * t3886;
    let t3888 = t1385 * t1385;
    let t3889 = t3887 * t3888;
    let t3897 = t3787 * t562;
    let t3898 = t3897 * t3793;
    let t3901 = t1338 * t1372;
    (t3880, t3882, t3887, t3888, t3889, t3898, t3901)
}
