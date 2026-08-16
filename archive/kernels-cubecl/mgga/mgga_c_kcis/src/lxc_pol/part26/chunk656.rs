//! MGGA_C_KCIS lxc pol — lxc_pol part 26 (v4rho3sigma_8) CSE chunk 656/1397 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part26_v4rho3sigma_8_chunk656<F: Float>(t1506: F, t7397: F, t20: F, t7052: F, t610: F, t4433: F, t6281: F, t4432: F, t1889: F, t2104: F, t4440: F, t4445: F) -> (F, F, F, F, F, F, F, F) {
    let t7398 = t1506 * t7397;
    let t7402 = t7052 * t20;
    let t7403 = t610 * t7402;
    let t7413 = t4433 * t6281;
    let t7414 = t4432 * t7413;
    let t7417 = t1889 * t2104;
    let t7418 = t4440 * t7417;
    let t7421 = t4445 * t6281;
    (t7398, t7402, t7403, t7413, t7414, t7417, t7418, t7421)
}
