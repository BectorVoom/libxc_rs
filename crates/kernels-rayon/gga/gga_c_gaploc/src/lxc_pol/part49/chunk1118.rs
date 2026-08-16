//! GGA_C_GAPLOC lxc pol — lxc_pol part 49 (v4rhosigma3_14) CSE chunk 1118/1217 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part49_v4rhosigma3_14_chunk1118(t41136: f64, t43619: f64, t43627: f64, t43630: f64, t43636: f64, t43640: f64, t43642: f64, t43645: f64, t43647: f64, t43648: f64, t43650: f64, t43653: f64) -> f64 {
    let t47280 = 0.15337170381568299871e1_f64 * t41136;
    let t47281 = t43619 + t43627 + t43630 + t43636 + t43640 + 0.11502877786176224903e2_f64 * t43642 + t43645 + t43647 - t43648 + 0.9585731488480187419e0_f64 * t43650 + t43653 - t47280;
    t47281
}
