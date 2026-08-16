//! GGA_C_GAPLOC lxc pol — lxc_pol part 38 (v4rhosigma3_3) CSE chunk 159/1003 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part38_v4rhosigma3_3_chunk159(t667: f64, t758: f64, t10: f64, t107: f64, t183: f64, t266: f64, t305: f64, t306: f64, t677: f64, t749: f64, t753: f64, t755: f64) -> f64 {
    let t759 = t758 * t667;
    let t768 = 0.58998125e-2_f64 * t749 * t306 - 0.11799625e-1_f64 * t753 * t755 - 0.58998125e-2_f64 * t305 * t759 - 0.14341111111111111111e-1_f64 * t107 * t10 * t266 - 0.21511666666666666667e-1_f64 * t107 * t183 * t677;
    t768
}
