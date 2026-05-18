//! MGGA_C_KCIS lxc pol — lxc_pol part 24 (v4rho3sigma_6) CSE chunk 820/1322 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part24_v4rho3sigma_6_chunk820<F: Float>(t169: F, t18443: F, t234: F, t1094: F, t6480: F, t1122: F, t1092: F, t6708: F, sigma0: F, zeta_threshold: F) -> (F, F, F, F, F, F) {
    let t170 = t169 <= zeta_threshold;
    let t18444 = piecewise3::<f64>(t170, F::new(0.0), t18443);
    let t18445 = t234 * t18444;
    let t18458 = t6480 * t1094;
    let t18459 = t18458 * sigma0;
    let t18460 = t18459 * t1122;
    let t18461 = t1092 * t18460;
    let t18463 = t6708 * sigma0;
    (t18444, t18445, t18458, t18459, t18461, t18463)
}
