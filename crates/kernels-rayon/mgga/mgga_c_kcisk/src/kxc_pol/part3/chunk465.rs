//! MGGA_C_KCISK kxc pol — kxc_pol part 3 (v3rho3_0) CSE chunk 465/1063 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcisk_kxc_pol_part3_v3rho3_0_chunk465(t1192: f64, t3634: f64, t1170: f64, t317: f64, t305: f64, t1190: f64, t1191: f64, t3571: f64, t303: f64, t3559: f64, t1180: f64, t3587: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t3636 = 2.0_f64 * t3634 * t1192;
    let t3637 = t1170 * t317;
    let t3638 = 1.0_f64 / t3637;
    let t3639 = t305 * t3638;
    let t3640 = t1190 * t1190;
    let t3641 = t3640 * t1191;
    let t3643 = 2.0_f64 * t3639 * t3641;
    let t3646 = 0.39862222222222222223e0_f64 * t3571;
    let t3651 = 1.0_f64/f64::sqrt(t303);
    let t3652 = t3651 * t3559;
    let t3654 = t1180 * t3587;
    (t3636, t3638, t3639, t3640, t3641, t3643, t3646, t3651, t3652, t3654)
}
