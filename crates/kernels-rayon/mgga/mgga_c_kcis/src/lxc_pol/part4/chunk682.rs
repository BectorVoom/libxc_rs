//! MGGA_C_KCIS lxc pol — lxc_pol part 4 (v3rho3_1) CSE chunk 682/1420 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part4_v3rho3_1_chunk682(t3797: f64, t3883: f64, t26: f64, t1330: f64, t3801: f64, t3805: f64, t3795: f64, t3799: f64, t3803: f64, t3807: f64, t3829: f64, t3831: f64, t3868: f64, t3874: f64, t3876: f64, t3880: f64, t3881: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t3884 = t3883 * t3797;
    let t3885 = t26 * t3884;
    let t3887 = t1330 * t3801;
    let t3888 = t26 * t3887;
    let t3890 = t1330 * t3805;
    let t3891 = t26 * t3890;
    let t3893 = -0.9494625e0_f64 * t3829 + 0.1898925e1_f64 * t3831 + t3868 + 0.19931111111111111111e0_f64 * t3795 - 0.19931111111111111111e0_f64 * t3799 + 0.59793333333333333334e0_f64 * t3803 - 0.29896666666666666667e0_f64 * t3807 + 0.15358125e0_f64 * t3874 + 0.3071625e0_f64 * t3876 + t3880 + 0.10954222222222222222e0_f64 * t3881 - 0.27385555555555555556e-1_f64 * t3885 + 0.16431333333333333333e0_f64 * t3888 - 0.82156666666666666667e-1_f64 * t3891;
    (t3884, t3885, t3887, t3888, t3890, t3891, t3893)
}
