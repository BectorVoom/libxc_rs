//! MGGA_C_KCIS lxc pol — lxc_pol part 26 (v4rho3sigma_8) CSE chunk 954/1397 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part26_v4rho3sigma_8_chunk954<F: Float>(t22059: F, t3786: F, t16029: F, t1961: F, t3255: F, t7226: F, t7218: F, t1889: F, t3761: F, t5481: F, t1897: F, t5477: F) -> (F, F, F, F, F, F, F) {
    let t22060 = t3786 * t22059;
    let t22063 = t16029 * t1961;
    let t22064 = t3786 * t22063;
    let t22067 = t3255 * t7226;
    let t22069 = t3255 * t7218;
    let t22072 = t3761 * t1889 * t5481;
    let t22076 = t3761 * t5477 * t1897;
    (t22060, t22063, t22064, t22067, t22069, t22072, t22076)
}
