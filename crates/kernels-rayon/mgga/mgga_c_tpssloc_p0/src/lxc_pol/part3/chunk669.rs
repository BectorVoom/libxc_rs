//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 3 (v3rho3_1) CSE chunk 669/1255 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part3_v3rho3_1_chunk669(t1156: f64, t3377: f64, t3236: f64, t3293: f64, t3238: f64, t3245: f64, t3250: f64, t3254: f64, t3272: f64, t3280: f64, t3288: f64, t3290: f64, t3295: f64, t3299: f64, t3302: f64, t3305: f64) -> (f64, f64, f64, f64) {
    let t3378 = t3377 * t1156;
    let t3383 = 0.40256666666666666667e0_f64 * t3236;
    let t3390 = 0.137975e0_f64 * t3293;
    let t3395 = -0.1294625e1_f64 * t3272 + 0.258925e1_f64 * t3280 + t3383 - 0.20128333333333333334e0_f64 * t3238 - 0.20128333333333333333e0_f64 * t3245 + 0.60385e0_f64 * t3250 + 0.301925e0_f64 * t3254 + 0.82524375e-1_f64 * t3288 + 0.16504875e0_f64 * t3290 + t3390 - 0.11038e0_f64 * t3295 - 0.27595e-1_f64 * t3299 + 0.16557e0_f64 * t3302 + 0.82785e-1_f64 * t3305;
    (t3378, t3383, t3390, t3395)
}
