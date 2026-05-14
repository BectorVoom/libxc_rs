//! MGGA_C_R2SCAN lxc pol — lxc_pol part 14 (v4rho3sigma_4) CSE chunk 841/1124 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part14_v4rho3sigma_4_chunk841<F: Float>(t1543: F, t797: F, t2259: F, t2330: F, t6897: F, t1234: F, t3264: F, t792: F, t1103: F, t1783: F, t1053: F, t1102: F, t357: F, t862: F, t255: F, t868: F) -> (F, F, F, F, F, F, F, F, F) {
    let t10611 = t797 * t1543;
    let t10622 = t797 * t2259;
    let t10626 = t6897 * t2330;
    let t10630 = t797 * t1234;
    let t10634 = t3264 * t792;
    let t10641 = t1103 * t1783;
    let t10643 = t1102 * t1053 * t10641;
    let t10645 = t862 * t357;
    let t10646 = t868 * t255;
    (t10611, t10622, t10626, t10630, t10634, t10641, t10643, t10645, t10646)
}
