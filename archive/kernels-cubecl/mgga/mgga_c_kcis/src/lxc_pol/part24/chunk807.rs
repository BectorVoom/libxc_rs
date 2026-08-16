//! MGGA_C_KCIS lxc pol — lxc_pol part 24 (v4rho3sigma_6) CSE chunk 807/1322 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part24_v4rho3sigma_6_chunk807<F: Float>(t14874: F, t1780: F, t245: F, t3393: F, t5155: F, t330: F, t4920: F, t5139: F, t5147: F, t8931: F, t2943: F, t365: F, sigma0: F) -> (F, F, F, F, F, F, F) {
    let t14875 = t14874 * sigma0;
    let t14907 = t1780 * t245;
    let t14913 = t3393 * t5155;
    let t14915 = t4920 * t330;
    let t14926 = F::cast_from(0.35374814814814814814e-1_f64) * t3393 * t5139;
    let t14927 = t8931 * t5147;
    let t14940 = t365 * t2943;
    (t14875, t14907, t14913, t14915, t14926, t14927, t14940)
}
