//! MGGA_C_R2SCAN lxc pol — lxc_pol part 15 (v4rho3sigma_5) CSE chunk 843/1115 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part15_v4rho3sigma_5_chunk843<F: Float>(t10634: F, t3276: F, t3262: F, t3424: F, t885: F, t1108: F, t1353: F, t1103: F, t1783: F, t1053: F, t1102: F, t357: F, t862: F, t255: F, t868: F, t258: F) -> (F, F, F, F, F, F, F, F, F) {
    let t10635 = t3276 * t10634;
    let t10636 = t3262 * t10635;
    let t10637 = 15.0 / 8.0 * t10636;
    let t10638 = t3424 * t885;
    let t10639 = 2.0 * t10638;
    let t10640 = t1353 * t1108;
    let t10641 = t1103 * t1783;
    let t10643 = t1102 * t1053 * t10641;
    let t10645 = t862 * t357;
    let t10646 = t868 * t255;
    let t10647 = t10646 * t258;
    (t10635, t10637, t10639, t10640, t10641, t10643, t10645, t10646, t10647)
}
