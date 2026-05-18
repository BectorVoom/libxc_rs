//! MGGA_C_RMGGAC lxc pol — lxc_pol part 13 (v4rho3sigma_4) CSE chunk 452/1127 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part13_v4rho3sigma_4_chunk452<F: Float>(t3885: F, t547: F, t821: F, t820: F, t98: F, t316: F, t815: F, t1579: F, t825: F, t101: F, t814: F, t154: F, t1583: F) -> (F, F, F, F, F, F) {
    let t4879 = t3885 * t547 * t821;
    let t4882 = t98 * t820;
    let t4883 = t815 * t316;
    let t4886 = t1579 * t825;
    let t4889 = t101 * t814;
    let t4892 = t1583 * t154;
    (t4879, t4882, t4883, t4886, t4889, t4892)
}
