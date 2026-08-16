//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 53 (v4rho2sigma2_9) CSE chunk 535/1059 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part53_v4rho2sigma2_9_chunk535(t3297: f64, t4724: f64, t136: f64, t1113: f64, t4729: f64, t4733: f64, t3238: f64, t3282: f64, t3294: f64, t3295: f64, t4721: f64, t4726: f64, t4731: f64, t4735: f64, t4749: f64, t4757: f64, t4765: f64, t4767: f64, t4770: f64) -> (f64, f64, f64, f64) {
    let t4772 = t3297 * t4724;
    let t4773 = t136 * t4772;
    let t4775 = t1113 * t4729;
    let t4776 = t136 * t4775;
    let t4778 = t1113 * t4733;
    let t4779 = t136 * t4778;
    let t4781 = -0.9494625e0_f64 * t4749 + 0.1898925e1_f64 * t4757 + t3282 - 0.99655555555555555557e-1_f64 * t3238 - 0.99655555555555555557e-1_f64 * t4721 - 0.19931111111111111111e0_f64 * t4726 + 0.59793333333333333334e0_f64 * t4731 + 0.29896666666666666667e0_f64 * t4735 + 0.15358125e0_f64 * t4765 + 0.3071625e0_f64 * t4767 + t3294 - 0.54771111111111111111e-1_f64 * t3295 - 0.54771111111111111111e-1_f64 * t4770 - 0.27385555555555555556e-1_f64 * t4773 + 0.16431333333333333333e0_f64 * t4776 + 0.82156666666666666667e-1_f64 * t4779;
    (t4773, t4776, t4779, t4781)
}
