//! MGGA_C_REVTPSS lxc pol — lxc_pol part 52 (v4rho2sigma2_7) CSE chunk 1262/1292 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part52_v4rho2sigma2_7_chunk1262(t109731: f64, t122407: f64, t125771: f64, t125775: f64, t128694: f64, t27845: f64, t27852: f64, t27857: f64, t27865: f64, t27896: f64, t27973: f64, t27981: f64, t28911: f64, t32690: f64, t32700: f64, t32719: f64, t34237: f64, t7303: f64) -> f64 {
    let t128767 = -0.17135921299530705785e1_f64 * t32700 * t34237 + 0.17347256376410398924e1_f64 * t32690 * t27896 - 0.56468933516960933999e-3_f64 * t125771 - 0.11423947533020470523e1_f64 * t32719 * t109731 * t7303 - 0.11423947533020470523e1_f64 * t32719 * t28911 * t27852 - 0.11423947533020470523e1_f64 * t32719 * t28911 * t27857 - 0.17347256376410398924e1_f64 * t128694 * t27981 - 0.17347256376410398924e1_f64 * t122407 * t27973 - 0.11423947533020470523e1_f64 * t32719 * t28911 * t27845 - 0.17347256376410398924e1_f64 * t122407 * t27865 + 0.7437465841810202164e-3_f64 * t125775;
    t128767
}
