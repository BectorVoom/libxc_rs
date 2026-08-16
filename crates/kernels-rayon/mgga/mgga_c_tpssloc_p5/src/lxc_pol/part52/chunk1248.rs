//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 52 (v4rho2sigma2_8) CSE chunk 1248/1400 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part52_v4rho2sigma2_8_chunk1248(t2717: f64, t6662: f64, t30642: f64, t6562: f64, t794: f64, t1902: f64, t213: f64, t225: f64, t30745: f64, t23030: f64, t30638: f64, t212: f64, t23171: f64, t6554: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t112873 = t2717 * t6662;
    let t112892 = t6562 * t794 * t30642;
    let t112899 = t213 * t1902 * t225;
    let t112908 = t30745 * t225;
    let t112936 = 0.52089578783527170489e-1_f64 * t23030 * t30638;
    let t112942 = 0.16449340668482264365e-1_f64 * t23171 * t212 * t1902 * t6554;
    (t112873, t112892, t112899, t112908, t112936, t112942)
}
