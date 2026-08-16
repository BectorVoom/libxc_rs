//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 28 (v4rho3sigma_4) CSE chunk 1223/2041 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1223(t2929: f64, t938: f64, t10523: f64, t315: f64, t10544: f64, t1004: f64, t3047: f64, t3053: f64, t3117: f64, t1043: f64, t676: f64, t248: f64, t884: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t10825 = t938 * t2929;
    let t10828 = t315 * t10523;
    let t10832 = 0.53272592592592592592e-1_f64 * t10544;
    let t10863 = t1004 * t3047;
    let t10866 = t3117 * t3053;
    let t10868 = t676 * t1043;
    let t10870 = t248 * t10868 * t884;
    (t10825, t10828, t10832, t10863, t10866, t10868, t10870)
}
