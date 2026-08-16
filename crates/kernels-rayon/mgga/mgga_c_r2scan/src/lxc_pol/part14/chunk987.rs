//! MGGA_C_R2SCAN lxc pol — lxc_pol part 14 (v4rho3sigma_4) CSE chunk 987/1276 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_r2scan_lxc_pol_part14_v4rho3sigma_4_chunk987(t3316: f64, t980: f64, t10868: f64, t2608: f64, t2147: f64, t1055: f64, t2834: f64, t20: f64, t5119: f64, t3293: f64, t2124: f64, t7406: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t11657 = t980 * t3316;
    let t11659 = t10868 * t2608;
    let t11660 = t2147 * t11659;
    let t11663 = t2834 * t1055;
    let t11669 = t5119 * t20;
    let t11670 = t3293 * t11669;
    let t11671 = t2124 * t7406;
    (t11657, t11659, t11660, t11663, t11669, t11670, t11671)
}
