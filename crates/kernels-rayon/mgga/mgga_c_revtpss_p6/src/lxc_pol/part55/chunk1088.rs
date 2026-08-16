//! MGGA_C_REVTPSS lxc pol — lxc_pol part 55 (v4rho2sigma2_10) CSE chunk 1088/1306 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part55_v4rho2sigma2_10_chunk1088(t1955: f64, t7997: f64, t1579: f64, t8651: f64, t31812: f64, t1568: f64, t3140: f64, t8477: f64, t1959: f64, t32434: f64, t32460: f64, t32473: f64, t32476: f64, t32480: f64, t32483: f64, t33675: f64, t33712: f64, t33719: f64, t7770: f64, t7775: f64, t8649: f64, t8652: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t34063 = t1955 * t7997;
    let t34068 = t8651 * t1579;
    let t34069 = t31812 * t34068;
    let t34074 = t1568 * t3140;
    let t34075 = t8477 * t34074;
    let t34078 = 0.17347256376410398924e1_f64 * t32434 * t7770 + t32460 - 0.3718732920905101082e-3_f64 * t33712 - t32473 + t32476 - 0.8673628188205199462e0_f64 * t34063 * t1959 + 0.7437465841810202164e-3_f64 * t33719 - 0.56468933516960933999e-3_f64 * t33675 + t32480 - t32483 - 0.17135921299530705785e1_f64 * t8649 * t34069 + 0.8673628188205199462e0_f64 * t32434 * t7775 + 0.57119737665102352616e0_f64 * t34075 * t8652;
    (t34063, t34068, t34069, t34074, t34075, t34078)
}
