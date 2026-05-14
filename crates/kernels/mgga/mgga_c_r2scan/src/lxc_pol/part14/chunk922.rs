//! MGGA_C_R2SCAN lxc pol — lxc_pol part 14 (v4rho3sigma_4) CSE chunk 922/1124 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part14_v4rho3sigma_4_chunk922<F: Float>(t11774: F, t10804: F, t10813: F, t10840: F, t11417: F, t11422: F, t11768: F, t12162: F, t12163: F, t12164: F, t12166: F, t11425: F, t11782: F, t11785: F, t11788: F, t11791: F, t11795: F, t11798: F, t11800: F, t11803: F, t11806: F, t11809: F, t11812: F) -> (F, F) {
    let t12167 = 0.10975748638225852664e-1 * t11774;
    let t12168 = t12162 + t12163 - t12164 - 0.97574405393827830187e-2 * t11768 - t12166 + t12167 + t10804 + t10813 - t11417 + t11422 - t10840;
    let t12180 = t11425 - 0.43663693315433241794e-2 * t11782 + 0.43663693315433241794e-2 * t11785 + 0.13099107994629972538e-1 * t11788 + 0.43663693315433241794e-2 * t11791 + 0.43663693315433241794e-2 * t11795 - 0.86682217400542685632e-1 * t11798 - 0.54878743191129263322e-1 * t11800 + 0.86682217400542685632e-1 * t11803 + 0.2600466522016280569e0 * t11806 + 0.86682217400542685632e-1 * t11809 + 0.2600466522016280569e0 * t11812;
    (t12168, t12180)
}
