//! MGGA_C_KCIS lxc pol — lxc_pol part 23 (v4rho3sigma_5) CSE chunk 155/1323 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part23_v4rho3sigma_5_chunk155<F: Float>(t187: F, t590: F, t629: F, t633: F, t103: F, t32: F, t5: F, t28: F, t14: F, t15: F) -> (F, F, F, F, F, F) {
    let t637 = t590 + t187 * (t629 * t633 - t590);
    let t643 = F::cast_from(0.11073577833333333333e-2_f64) * t5 * t103 * t32;
    let t644 = t28 * t28;
    let t645 = F::cast_from(1.0_f64) / t644;
    let t646 = t14 * t645;
    let t647 = F::cast_from(1.0_f64) / t15;
    (t637, t643, t644, t645, t646, t647)
}
