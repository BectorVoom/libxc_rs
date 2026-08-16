//! MGGA_C_KCISK lxc pol — lxc_pol part 6 (v3rho3_3) CSE chunk 466/1086 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcisk_lxc_pol_part6_v3rho3_3_chunk466(t167: f64, t3532: f64, t408: f64, t1218: f64, t411: f64, t338: f64, t389: f64, t394: f64, t123: f64, t6: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t3891 = t167 * t3532;
    let t3923 = t408 * t408;
    let t3924 = 1.0_f64 / t3923;
    let t3929 = 1.0_f64 / t1218 / t411;
    let t3930 = t338 * t3929;
    let t3933 = t389 * t394;
    let t3934 = t123 * t6;
    (t3891, t3923, t3924, t3929, t3930, t3933, t3934)
}
