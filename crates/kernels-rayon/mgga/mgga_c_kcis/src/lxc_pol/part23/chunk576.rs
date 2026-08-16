//! MGGA_C_KCIS lxc pol — lxc_pol part 23 (v4rho3sigma_5) CSE chunk 576/1323 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part23_v4rho3sigma_5_chunk576(t3883: f64, t5427: f64, t26: f64, t1330: f64, t5441: f64, t5477: f64, t4714: f64, t3795: f64, t3868: f64, t3880: f64, t3881: f64, t5469: f64, t5472: f64, t5475: f64, t5479: f64, t5514: f64, t5516: f64, t5557: f64, t5559: f64, t5562: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t5564 = t3883 * t5427;
    let t5565 = t26 * t5564;
    let t5567 = t1330 * t5441;
    let t5568 = t26 * t5567;
    let t5570 = t1330 * t5477;
    let t5571 = t4714 * t5570;
    let t5573 = -0.9494625e0_f64 * t5514 + 0.1898925e1_f64 * t5516 + t3868 + 0.99655555555555555557e-1_f64 * t3795 + 0.99655555555555555557e-1_f64 * t5469 - 0.19931111111111111111e0_f64 * t5472 + 0.59793333333333333334e0_f64 * t5475 + 0.59793333333333333334e0_f64 * t5479 + 0.15358125e0_f64 * t5557 + 0.3071625e0_f64 * t5559 + t3880 + 0.54771111111111111111e-1_f64 * t3881 + 0.54771111111111111111e-1_f64 * t5562 - 0.27385555555555555556e-1_f64 * t5565 + 0.16431333333333333333e0_f64 * t5568 + 0.16431333333333333333e0_f64 * t5571;
    (t5564, t5565, t5567, t5568, t5570, t5571, t5573)
}
