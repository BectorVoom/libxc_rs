//! MGGA_C_REVTPSS lxc pol — lxc_pol part 52 (v4rho2sigma2_7) CSE chunk 1093/1292 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part52_v4rho2sigma2_7_chunk1093(t2030: f64, t32681: f64, t32682: f64, t32683: f64, t32709: f64, t32712: f64, t32718: f64, t32719: f64, t33923: f64, t33931: f64, t33960: f64, t33967: f64, t34204: f64, t34212: f64, t7930: f64, t8702: f64) -> f64 {
    let t34216 = -0.8673628188205199462e0_f64 * t34204 * t2030 + t32681 + t32682 - t32683 - 0.3718732920905101082e-3_f64 * t33960 - 0.56468933516960933999e-3_f64 * t33931 - 0.56468933516960933999e-3_f64 * t33923 - 0.8673628188205199462e0_f64 * t8702 * t7930 - 0.11423947533020470523e1_f64 * t32719 * t34212 - t32709 + t32712 + 0.7437465841810202164e-3_f64 * t33967 - t32718;
    t34216
}
