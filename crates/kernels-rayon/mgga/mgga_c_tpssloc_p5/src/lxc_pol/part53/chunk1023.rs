//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 53 (v4rho2sigma2_9) CSE chunk 1023/1059 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part53_v4rho2sigma2_9_chunk1023(t123452: f64, t123476: f64, t123503: f64, t123521: f64, t123552: f64, t123612: f64, t123687: f64, t123711: f64, t870: f64, t2752: f64, t33990: f64, t1877: f64, t2219: f64, t8748: f64) -> (f64, f64, f64, f64) {
    let t123714 = t123452 + t123476 + t123503 + t123521 + t123552 + t123612 + t123687 + t123711;
    let t123715 = t123714 * t870;
    let t123719 = t33990 * t2752;
    let t123733 = t1877 * t8748 * t2219;
    (t123714, t123715, t123719, t123733)
}
