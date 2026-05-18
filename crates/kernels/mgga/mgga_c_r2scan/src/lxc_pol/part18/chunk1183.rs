//! MGGA_C_R2SCAN lxc pol — lxc_pol part 18 (v4rho3sigma_8) CSE chunk 1183/1264 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part18_v4rho3sigma_8_chunk1183<F: Float>(t1577: F, t3308: F, t9547: F, t39549: F, t41439: F, t43115: F, t43117: F, t43120: F, t43123: F, t43126: F, t43130: F, t43133: F, t43135: F, t43138: F) -> F {
    let t43141 = t1577 * t3308 * t9547;
    let t43143 = F::new(0.54878743191129263322e-1) * t43115 - t39549 - F::new(0.54878743191129263322e-2) * t43117 - t41439 - F::new(0.13099107994629972538e-1) * t43120 + F::new(0.43663693315433241792e-2) * t43123 + F::new(0.21831846657716620896e-2) * t43126 + F::new(0.21831846657716620896e-2) * t43130 - F::new(0.13972381860938637374e0) * t43133 + F::new(0.2600466522016280569e0) * t43135 + F::new(0.13002332610081402845e0) * t43138 + F::new(0.26004665220162805689e0) * t43141;
    t43143
}
