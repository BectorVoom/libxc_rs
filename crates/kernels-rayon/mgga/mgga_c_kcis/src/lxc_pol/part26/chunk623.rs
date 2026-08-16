//! MGGA_C_KCIS lxc pol — lxc_pol part 26 (v4rho3sigma_8) CSE chunk 623/1397 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part26_v4rho3sigma_8_chunk623(t1317: f64, t6964: f64, t3873: f64, t6957: f64, t1324: f64, t3883: f64, t6937: f64, t26: f64, t1330: f64, t6912: f64, t6944: f64, t3868: f64, t3880: f64, t5469: f64, t5562: f64, t6939: f64, t6942: f64, t6946: f64, t6958: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t6965 = t1317 * t6964;
    let t6971 = t3873 * t6957;
    let t6973 = t1324 * t6964;
    let t6976 = t3883 * t6937;
    let t6977 = t26 * t6976;
    let t6979 = t1330 * t6912;
    let t6980 = t26 * t6979;
    let t6982 = t1330 * t6944;
    let t6983 = t26 * t6982;
    let t6985 = -0.9494625e0_f64 * t6958 + 0.1898925e1_f64 * t6965 + t3868 + 0.19931111111111111111e0_f64 * t5469 - 0.19931111111111111111e0_f64 * t6939 + 0.59793333333333333334e0_f64 * t6942 - 0.29896666666666666667e0_f64 * t6946 + 0.15358125e0_f64 * t6971 + 0.3071625e0_f64 * t6973 + t3880 + 0.10954222222222222222e0_f64 * t5562 - 0.27385555555555555556e-1_f64 * t6977 + 0.16431333333333333333e0_f64 * t6980 - 0.82156666666666666667e-1_f64 * t6983;
    (t6965, t6971, t6973, t6976, t6977, t6979, t6980, t6982, t6983, t6985)
}
