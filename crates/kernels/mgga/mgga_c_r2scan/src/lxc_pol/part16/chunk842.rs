//! MGGA_C_R2SCAN lxc pol — lxc_pol part 16 (v4rho3sigma_6) CSE chunk 842/1112 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part16_v4rho3sigma_6_chunk842<F: Float>(t10666: F, t97: F, t1065: F, t481: F, t3270: F, t2104: F, t3436: F, t2302: F) -> (F, F, F) {
    let t10667 = t97 * t10666;
    let t10668 = t1065 * t481;
    let t10669 = t3270 * t10668;
    let t10672 = t2104 * t3436;
    let t10673 = t2302 * t10672;
    (t10667, t10669, t10673)
}
