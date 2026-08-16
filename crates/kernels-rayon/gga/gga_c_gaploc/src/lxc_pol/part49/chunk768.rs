//! GGA_C_GAPLOC lxc pol — lxc_pol part 49 (v4rhosigma3_14) CSE chunk 768/1217 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part49_v4rhosigma3_14_chunk768(t13034: f64, t5748: f64, t2949: f64, t3209: f64, t1445: f64, t813: f64, t2958: f64, t833: f64, t2097: f64, t3039: f64, t3277: f64, t12658: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t13036 = 0.27606906686822939767e2_f64 * t5748 * t13034;
    let t13037 = t2949 * t3209;
    let t13038 = t1445 * t13037;
    let t13040 = 0.92023022289409799224e1_f64 * t813 * t13038;
    let t13041 = t2958 * t3209;
    let t13042 = t1445 * t13041;
    let t13044 = 0.43710935587469654631e2_f64 * t833 * t13042;
    let t13045 = t3039 * t2097;
    let t13047 = 0.25025342966295298669e1_f64 * t3277 * t13045;
    let t13050 = 0.11502877786176224903e1_f64 * t12658;
    (t13036, t13037, t13038, t13040, t13041, t13042, t13044, t13045, t13047, t13050)
}
