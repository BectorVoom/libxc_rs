//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 31 (v4rho3sigma_7) CSE chunk 1850/2041 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1850<F: Float>(t87931: F, t10143: F, t7844: F, t27143: F, t532: F, t90459: F, t90468: F, t90470: F, t90472: F, t225: F, t27137: F, t27059: F) -> (F, F, F, F, F, F, F, F, F) {
    let t92976 = F::cast_from(0.15352717957250113407e0_f64) * t87931;
    let t93000 = t7844 * t10143;
    let t93286 = t532 * t27143;
    let t93306 = F::cast_from(0.76763589786250567036e-1_f64) * t90459;
    let t93309 = F::cast_from(0.15352717957250113407e0_f64) * t90468;
    let t93310 = F::cast_from(0.15352717957250113407e0_f64) * t90470;
    let t93311 = F::cast_from(0.15352717957250113407e0_f64) * t90472;
    let t93313 = t27137 * t225;
    let t93316 = t27059 * t225;
    (t92976, t93000, t93286, t93306, t93309, t93310, t93311, t93313, t93316)
}
