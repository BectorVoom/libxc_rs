//! MGGA_C_REVTPSS lxc pol — lxc_pol part 51 (v4rho2sigma2_6) CSE chunk 992/1200 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part51_v4rho2sigma2_6_chunk992(t31808: f64, t31829: f64, t31833: f64, t31850: f64, t33695: f64, t33699: f64, t33704: f64, t33708: f64, t33712: f64, t33717: f64, t33719: f64, t33723: f64, t8481: f64, t8649: f64) -> f64 {
    let t33725 = t31808 + 0.57119737665102352616e0_f64 * t33695 * t8481 - 0.17135921299530705785e1_f64 * t8649 * t33699 - 0.11423947533020470523e1_f64 * t8649 * t33704 + 0.11423947533020470523e1_f64 * t8649 * t33708 + t31829 - t31833 - 0.1859366460452550541e-3_f64 * t33712 + 0.3718732920905101082e-3_f64 * t33717 + 0.3718732920905101082e-3_f64 * t33719 + t31850 + 0.7437465841810202164e-3_f64 * t33723;
    t33725
}
