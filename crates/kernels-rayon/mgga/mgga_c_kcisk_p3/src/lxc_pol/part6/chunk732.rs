//! MGGA_C_KCISK lxc pol — lxc_pol part 6 (v3rho3_3) CSE chunk 732/1086 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcisk_lxc_pol_part6_v3rho3_3_chunk732(t13522: f64, t370: f64, t4125: f64, t13665: f64, t1389: f64, t381: f64, t916: f64, t1318: f64, t398: f64, t13614: f64, t397: f64, t403: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t13686 = 0.28842592592592592592e-1_f64 * t13522;
    let t13715 = 1.0_f64 / t4125 / t370;
    let t13746 = 0.73586666666666666667e0_f64 * t13665;
    let t13748 = 0.93932222222222222223e0_f64 * t13522;
    let t13776 = 1.0_f64 / t381 / t916 / t1389;
    let t13829 = t1318 * t1318;
    let t13830 = 1.0_f64 / t13829;
    let t13831 = t398 * t13830;
    let t13871 = t397 * t13614 * t403;
    (t13686, t13715, t13746, t13748, t13776, t13831, t13871)
}
