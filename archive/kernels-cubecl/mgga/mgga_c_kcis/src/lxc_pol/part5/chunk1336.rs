//! MGGA_C_KCIS lxc pol — lxc_pol part 5 (v3rho3_2) CSE chunk 1336/1419 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part5_v3rho3_2_chunk1336<F: Float>(t3255: F, t7214: F, t518: F, t7141: F, t1319: F, t3786: F, t1419: F, t7142: F, t5498: F, t1889: F, t3766: F, t5526: F) -> (F, F, F, F, F, F) {
    let t22091 = t3255 * t7214;
    let t22093 = t518 * t7141;
    let t22094 = t22093 * t1319;
    let t22095 = t3786 * t22094;
    let t22098 = t7142 * t1419;
    let t22099 = t5498 * t22098;
    let t22103 = t3766 * t1889 * t5526;
    (t22091, t22094, t22095, t22098, t22099, t22103)
}
