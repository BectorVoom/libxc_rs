//! MGGA_C_KCIS lxc pol — lxc_pol part 5 (v3rho3_2) CSE chunk 857/1419 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part5_v3rho3_2_chunk857<F: Float>(t1317: F, t6964: F, t3873: F, t6957: F, t1324: F, t3883: F, t6937: F, t26: F, t1330: F, t6912: F, t6944: F, t3868: F, t3880: F, t5469: F, t5562: F, t6939: F, t6942: F, t6946: F, t6958: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t6965 = t1317 * t6964;
    let t6971 = t3873 * t6957;
    let t6973 = t1324 * t6964;
    let t6976 = t3883 * t6937;
    let t6977 = t26 * t6976;
    let t6979 = t1330 * t6912;
    let t6980 = t26 * t6979;
    let t6982 = t1330 * t6944;
    let t6983 = t26 * t6982;
    let t6985 = -F::new(0.9494625e0) * t6958 + F::new(0.1898925e1) * t6965 + t3868 + F::new(0.19931111111111111111e0) * t5469 - F::new(0.19931111111111111111e0) * t6939 + F::new(0.59793333333333333334e0) * t6942 - F::new(0.29896666666666666667e0) * t6946 + F::new(0.15358125e0) * t6971 + F::new(0.3071625e0) * t6973 + t3880 + F::new(0.10954222222222222222e0) * t5562 - F::new(0.27385555555555555556e-1) * t6977 + F::new(0.16431333333333333333e0) * t6980 - F::new(0.82156666666666666667e-1) * t6983;
    (t6965, t6971, t6973, t6976, t6977, t6979, t6980, t6982, t6983, t6985)
}
