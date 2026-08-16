//! MGGA_C_REVTPSS lxc pol — lxc_pol part 56 (v4rho2sigma2_11) CSE chunk 979/1203 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part56_v4rho2sigma2_11_chunk979(t1265: f64, t31993: f64, t1238: f64, t1252: f64, t33469: f64, t33471: f64, t33474: f64, t33477: f64, t33480: f64, t33484: f64, t33487: f64, t33491: f64, t33495: f64, t33498: f64, t33502: f64, t33505: f64, t33509: f64, t33512: f64, t33518: f64, t33523: f64, t33524: f64, t8941: f64, t8948: f64) -> (f64, f64) {
    let t33525 = t31993 * t1265;
    let t33528 = -0.17135921299530705785e1_f64 * t33469 * t33471 + 0.57119737665102352616e0_f64 * t33474 * t8941 - 0.17135921299530705785e1_f64 * t33477 * t33480 + 0.11423947533020470523e1_f64 * t33484 * t33487 + 0.11423947533020470523e1_f64 * t33477 * t33491 - 0.5578099381357651623e-3_f64 * t33495 * t33498 + 0.5578099381357651623e-3_f64 * t33502 * t1238 - 0.1859366460452550541e-3_f64 * t33505 * t8948 + 0.3718732920905101082e-3_f64 * t33509 * t33512 - 0.3718732920905101082e-3_f64 * t33518 * t1252 - t33523 + 0.12395776403017003607e-3_f64 * t33524 * t33525;
    (t33525, t33528)
}
