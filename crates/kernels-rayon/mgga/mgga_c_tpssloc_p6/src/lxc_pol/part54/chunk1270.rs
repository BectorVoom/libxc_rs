//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 54 (v4rho2sigma2_10) CSE chunk 1270/1484 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part54_v4rho2sigma2_10_chunk1270(t1902: f64, t213: f64, t225: f64, t23030: f64, t30638: f64, t212: f64, t23171: f64, t6554: f64, t794: f64, t23164: f64, t6555: f64, t6562: f64, t6572: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t112899 = t213 * t1902 * t225;
    let t112936 = 0.52089578783527170489e-1_f64 * t23030 * t30638;
    let t112942 = 0.16449340668482264365e-1_f64 * t23171 * t212 * t1902 * t6554;
    let t112943 = t794 * t1902;
    let t112945 = t23164 * t112943 * t6555;
    let t112948 = t6562 * t112943 * t6572;
    (t112899, t112936, t112942, t112943, t112945, t112948)
}
