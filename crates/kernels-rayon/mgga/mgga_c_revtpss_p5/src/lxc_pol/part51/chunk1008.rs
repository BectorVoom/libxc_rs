//! MGGA_C_REVTPSS lxc pol — lxc_pol part 51 (v4rho2sigma2_6) CSE chunk 1008/1200 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part51_v4rho2sigma2_6_chunk1008(t32246: f64, t32267: f64, t32271: f64, t32288: f64, t33943: f64, t33947: f64, t33952: f64, t33956: f64, t33960: f64, t33965: f64, t33967: f64, t33971: f64, t8586: f64, t8706: f64) -> f64 {
    let t33973 = t32246 + 0.57119737665102352616e0_f64 * t33943 * t8586 - 0.17135921299530705785e1_f64 * t8706 * t33947 - 0.11423947533020470523e1_f64 * t8706 * t33952 + 0.11423947533020470523e1_f64 * t8706 * t33956 + t32267 - t32271 - 0.1859366460452550541e-3_f64 * t33960 + 0.3718732920905101082e-3_f64 * t33965 + 0.3718732920905101082e-3_f64 * t33967 + t32288 + 0.7437465841810202164e-3_f64 * t33971;
    t33973
}
