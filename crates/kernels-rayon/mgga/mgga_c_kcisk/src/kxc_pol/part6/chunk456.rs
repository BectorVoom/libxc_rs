//! MGGA_C_KCISK kxc pol — kxc_pol part 6 (v3rho3_3) CSE chunk 456/1086 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcisk_kxc_pol_part6_v3rho3_3_chunk456(t3598: f64, t420: f64, t3571: f64, t1170: f64, t317: f64, t305: f64, t303: f64, t1379: f64, t311: f64, t313: f64, t1311: f64, t79: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t3599 = t3598 * t420;
    let t3611 = 0.55033333333333333333e-2_f64 * t3571;
    let t3626 = 0.23744444444444444444e-1_f64 * t3571;
    let t3637 = t1170 * t317;
    let t3638 = 1.0_f64 / t3637;
    let t3639 = t305 * t3638;
    let t3646 = 0.39862222222222222223e0_f64 * t3571;
    let t3651 = 1.0_f64/f64::sqrt(t303);
    let t3657 = t311 * t1379 * t313;
    let t3658 = 0.13692777777777777778e0_f64 * t3657;
    let t3661 = t79 * t1311;
    (t3599, t3611, t3626, t3638, t3639, t3646, t3651, t3657, t3658, t3661)
}
