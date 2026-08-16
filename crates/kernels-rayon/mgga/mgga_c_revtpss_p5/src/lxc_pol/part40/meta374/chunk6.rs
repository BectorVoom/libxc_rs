//! MGGA_C_REVTPSS lxc pol — lxc_pol part 40 (v4rho3tau_3) CSE chunk 1328/1507 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part40_v4rho3tau_3_chunk1328(t15140: f64, t16012: f64, t15780: f64, t4900: f64, t3117: f64, t3133: f64, t357: f64, t4893: f64, t3059: f64, t4781: f64, t1011: f64, t11927: f64, t11933: f64, t15996: f64, t15997: f64, t16000: f64, t16003: f64, t16006: f64, t16009: f64, t4899: f64, t4907: f64, t4912: f64) -> f64 {
    let t16013 = t16012 * t15140;
    let t16016 = t15780 * t4900;
    let t16017 = t3117 * t16016;
    let t16020 = t3133 * t357;
    let t16021 = t4893 * t16020;
    let t16022 = t3117 * t16021;
    let t16025 = t357 * t3059;
    let t16026 = t4781 * t16025;
    let t16027 = t3117 * t16026;
    let t16034 = t15996 - t1011 * t15997 / 72.0_f64 - t1011 * t16000 / 144.0_f64 - t1011 * t16003 / 36.0_f64 + t1011 * t16006 / 108.0_f64 + t1011 * t16009 / 216.0_f64 + 7.0_f64 / 648.0_f64 * t1011 * t16013 - 0.42874018118069736972e-3_f64 * t4899 * t16017 - 0.21437009059034868486e-3_f64 * t4899 * t16022 + 0.42874018118069736972e-3_f64 * t11927 * t16027 + 0.22866142996303859718e-2_f64 * t11933 * t4912 + 0.22866142996303859718e-2_f64 * t11933 * t4907;
    t16034
}
