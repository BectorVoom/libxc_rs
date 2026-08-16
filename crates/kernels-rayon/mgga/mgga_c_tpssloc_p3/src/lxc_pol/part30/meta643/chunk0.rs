//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 30 (v4rho3sigma_6) CSE chunk 2053/2341 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk2053(t23562: f64, t343: f64, t88405: f64, t1036: f64, t25622: f64, t14134: f64, t6765: f64, t1933: f64, t23479: f64, t88360: f64, t88365: f64, t25637: f64, t984: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t88407 = t23562 * t88405 * t343;
    let t88415 = t25622 * t1036 / 216.0_f64;
    let t88422 = t6765 * t14134 / 864.0_f64;
    let t88425 = 0.20186378047070195428e-3_f64 * t1933 * t88360 * t23479;
    let t88428 = 0.20186378047070195428e-3_f64 * t1933 * t88365 * t23479;
    let t88430 = t23562 * t25637 * t984;
    (t88407, t88415, t88422, t88425, t88428, t88430)
}
