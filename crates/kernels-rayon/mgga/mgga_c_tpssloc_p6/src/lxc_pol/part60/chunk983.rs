//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 60 (v4rho2sigma2_16) CSE chunk 983/1064 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part60_v4rho2sigma2_16_chunk983(t22759: f64, t6388: f64, t6936: f64, t1985: f64, t214: f64, t225: f64, t28107: f64, t567: f64, t120308: f64, t120544: f64, t7700: f64, t120532: f64) -> (f64, f64, f64, f64, f64) {
    let t127299 = t6936 * t22759 * t6388;
    let t127316 = 0.16449340668482264365e-1_f64 * t1985 * t214 * t28107 * t225 * t567;
    let t127325 = 0.3289868133696452873e-1_f64 * t120308;
    let t127328 = 0.3289868133696452873e-1_f64 * t1985 * t120544 * t7700;
    let t127346 = 0.76763589786250567036e-1_f64 * t120532;
    (t127299, t127316, t127325, t127328, t127346)
}
