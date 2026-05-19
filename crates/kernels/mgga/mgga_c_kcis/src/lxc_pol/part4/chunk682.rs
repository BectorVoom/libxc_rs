//! MGGA_C_KCIS lxc pol — lxc_pol part 4 (v3rho3_1) CSE chunk 682/1420 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part4_v3rho3_1_chunk682<F: Float>(t3797: F, t3883: F, t26: F, t1330: F, t3801: F, t3805: F, t3795: F, t3799: F, t3803: F, t3807: F, t3829: F, t3831: F, t3868: F, t3874: F, t3876: F, t3880: F, t3881: F) -> (F, F, F, F, F, F, F) {
    let t3884 = t3883 * t3797;
    let t3885 = t26 * t3884;
    let t3887 = t1330 * t3801;
    let t3888 = t26 * t3887;
    let t3890 = t1330 * t3805;
    let t3891 = t26 * t3890;
    let t3893 = -F::new(0.9494625e0) * t3829 + F::new(0.1898925e1) * t3831 + t3868 + F::cast_from(0.19931111111111111111e0_f64) * t3795 - F::cast_from(0.19931111111111111111e0_f64) * t3799 + F::cast_from(0.59793333333333333334e0_f64) * t3803 - F::cast_from(0.29896666666666666667e0_f64) * t3807 + F::new(0.15358125e0) * t3874 + F::new(0.3071625e0) * t3876 + t3880 + F::cast_from(0.10954222222222222222e0_f64) * t3881 - F::cast_from(0.27385555555555555556e-1_f64) * t3885 + F::cast_from(0.16431333333333333333e0_f64) * t3888 - F::cast_from(0.82156666666666666667e-1_f64) * t3891;
    (t3884, t3885, t3887, t3888, t3890, t3891, t3893)
}
