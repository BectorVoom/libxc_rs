//! MGGA_C_KCISK lxc pol — lxc_pol part 25 (v4rho3sigma_5) CSE chunk 686/1395 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part25_v4rho3sigma_5_chunk686<F: Float>(t6707: F, t7303: F, t7302: F, t5290: F, t6702: F, t5289: F, t5320: F, t739: F, t5330: F, t79: F, t5283: F, t718: F) -> (F, F, F, F, F, F, F, F, F) {
    let t7304 = t7303 * t6707;
    let t7305 = t7302 * t7304;
    let t7307 = t5290 * t6702;
    let t7308 = t5289 * t7307;
    let t7310 = t739 * t5320;
    let t7311 = t79 * t5330;
    let t7312 = t7311 * t6707;
    let t7313 = t7310 * t7312;
    let t7315 = t5283 * t718;
    (t7304, t7305, t7307, t7308, t7310, t7311, t7312, t7313, t7315)
}
