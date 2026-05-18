//! MGGA_C_KCIS lxc pol — lxc_pol part 24 (v4rho3sigma_6) CSE chunk 205/1322 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part24_v4rho3sigma_6_chunk205<F: Float>(t169: F, t279: F, t829: F, t234: F, t237: F, t240: F, t318: F, sigma0: F, zeta_threshold: F) -> (F, F, F, F, F) {
    let t170 = t169 <= zeta_threshold;
    let t910 = F::new(1.0) / t279;
    let t911 = sigma0 * t910;
    let t914 = piecewise3::<f64>(t170, F::new(0.0), t829);
    let t915 = t234 * t914;
    let t920 = t237 * t318 * t240;
    (t910, t911, t914, t915, t920)
}
