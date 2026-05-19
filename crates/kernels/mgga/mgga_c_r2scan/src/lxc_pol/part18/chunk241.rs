//! MGGA_C_R2SCAN lxc pol — lxc_pol part 18 (v4rho3sigma_8) CSE chunk 241/1264 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part18_v4rho3sigma_8_chunk241<F: Float>(t735: F, t736: F, t224: F, t704: F, t225: F, t695: F) -> (F, F, F, F) {
    let t738 = F::cast_from(0.54217906501508699211e-2_f64) * t735 * t736;
    let t739 = t704 * t224;
    let t740 = t225 * t695;
    let t741 = t739 * t740;
    (t738, t739, t740, t741)
}
