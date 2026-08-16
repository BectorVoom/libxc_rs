//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 27 (v4rho3sigma_3) CSE chunk 2211/2372 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk2211(t25651: f64, t3: f64, t83120: f64, t1409: f64, t984: f64, t23562: f64, t343: f64, t1036: f64, t25622: f64, t14134: f64, t6765: f64, t1933: f64, t23479: f64, t88360: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t88400 = t83120 * t3 * t25651;
    let t88405 = t1409 * t984;
    let t88407 = t23562 * t88405 * t343;
    let t88415 = t25622 * t1036 / 216.0_f64;
    let t88422 = t6765 * t14134 / 864.0_f64;
    let t88425 = 0.20186378047070195428e-3_f64 * t1933 * t88360 * t23479;
    (t88400, t88405, t88407, t88415, t88422, t88425)
}
