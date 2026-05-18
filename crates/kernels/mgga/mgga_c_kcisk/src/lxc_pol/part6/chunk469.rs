//! MGGA_C_KCISK lxc pol — lxc_pol part 6 (v3rho3_3) CSE chunk 469/1086 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part6_v3rho3_3_chunk469<F: Float>(t3532: F, t403: F, t1318: F, t402: F, t398: F, t1311: F, t25: F) -> (F, F, F, F) {
    let t3953 = t403 * t3532;
    let t3959 = F::new(1.0) / t1318 / t402;
    let t3960 = t398 * t3959;
    let t3973 = t25 * t1311;
    (t3953, t3959, t3960, t3973)
}
