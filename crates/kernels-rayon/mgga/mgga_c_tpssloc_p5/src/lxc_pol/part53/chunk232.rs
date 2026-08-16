//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 53 (v4rho2sigma2_9) CSE chunk 232/1059 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part53_v4rho2sigma2_9_chunk232(t950: f64, t951: f64, t300: f64, t311: f64, t890: f64, t916: f64, t919: f64, t924: f64, t933: f64, t939: f64, t943: f64, t315: f64) -> (f64, f64, f64, f64) {
    let t952 = t950 * t951;
    let t956 = t300 * (-0.310907e-1_f64 * t919 * t311 + 1.0_f64 * t924 * t933 + t890 - t916 - 0.19751673498613801407e-1_f64 * t939 + 0.5848223622634646207e0_f64 * t943 * t952);
    let t958 = 0.19751673498613801407e-1_f64 * t300 * t939;
    let t959 = t300 * t315;
    (t952, t956, t958, t959)
}
