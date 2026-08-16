//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 45 (v4rho2sigma2_1) CSE chunk 935/1056 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part45_v4rho2sigma2_1_chunk935(t1902: f64, t212: f64, t23171: f64, t6554: f64, t794: f64, t23164: f64, t6555: f64, t6562: f64, t6572: f64, t234: f64, t6624: f64, t6552: f64, t6637: f64, t776: f64) -> (f64, f64, f64, f64) {
    let t112942 = 0.16449340668482264365e-1_f64 * t23171 * t212 * t1902 * t6554;
    let t112943 = t794 * t1902;
    let t112945 = t23164 * t112943 * t6555;
    let t112946 = 0.3289868133696452873e-1_f64 * t112945;
    let t112948 = t6562 * t112943 * t6572;
    let t112949 = 0.16449340668482264365e-1_f64 * t112948;
    let t112951 = t234 * t6624;
    let t112955 = 0.6579736267392905746e-1_f64 * t6552 * t6637 * t112951 * t776;
    (t112942, t112946, t112949, t112955)
}
