//! MGGA_C_R2SCAN lxc pol — lxc_pol part 15 (v4rho3sigma_5) CSE chunk 792/1253 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part15_v4rho3sigma_5_chunk792<F: Float>(t2321: F, t955: F, t6897: F, t986: F, t1048: F, t2330: F, t1543: F, t2854: F, t2858: F, t4987: F, t4938: F, t889: F) -> (F, F, F, F, F, F) {
    let t6966 = t2321 * t955;
    let t6967 = t986 * t6897;
    let t6969 = t1048 * t6967 * t2330;
    let t6970 = F::new(2.0) * t6969;
    let t6972 = t2858 * t2854 * t1543;
    let t6973 = F::new(6.0) * t6972;
    let t6975 = F::new(0.34631718211362927518e2) * t4987;
    let t6976 = t4938 * t889;
    (t6966, t6967, t6970, t6973, t6975, t6976)
}
