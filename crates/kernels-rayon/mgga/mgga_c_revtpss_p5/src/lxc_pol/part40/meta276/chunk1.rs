//! MGGA_C_REVTPSS lxc pol — lxc_pol part 40 (v4rho3tau_3) CSE chunk 1018/1507 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part40_v4rho3tau_3_chunk1018(t2630: f64, t9860: f64, t2516: f64, t676: f64, t3869: f64, t2496: f64, t3926: f64, t3930: f64, t221: f64, t4019: f64, t4057: f64, t4018: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t9861 = t9860 * t2630;
    let t9863 = t676 * t2516;
    let t9865 = 0.16265371950452609763e-1_f64 * t3869 * t9863;
    let t9866 = t676 * t2496;
    let t9868 = 0.48159733137676571078e0_f64 * t3869 * t9866;
    let t9896 = t3930 * t3926;
    let t9905 = t4019 * t221 * t4057;
    let t9906 = t4018 * t9905;
    (t9861, t9863, t9865, t9866, t9868, t9896, t9906)
}
