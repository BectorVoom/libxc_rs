//! MGGA_C_R2SCAN lxc pol — lxc_pol part 8 (v4rho4_3) CSE chunk 1382/1467 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part8_v4rho4_3_chunk1382<F: Float>(t21874: F, t21884: F, t21887: F, t21899: F, t21902: F, t22395: F, t22399: F, t22403: F, t22404: F, t22409: F, t26765: F, t28794: F, t28802: F, t21914: F, t21923: F, t21927: F, t21930: F, t22411: F, t22416: F, t22424: F, t26771: F, t26773: F, t26783: F, t26788: F, t28805: F, t28808: F, t28815: F) -> (F, F) {
    let t33653 = -t22395 - t22399 + t22403 + 12.0 * t28794 + t21874 - t21884 - t21887 + t21899 + 12.0 * t22404 + t21902 - 3.0 * t28802 + t26765 - t22409;
    let t33662 = -0.3903689268108626343e0 * t28805 - 0.254044196e-2 * t22411 - t22416 - 0.32530743900905219526e-1 * t28808 - t21914 - t26771 - t21923 + 24.0 * t22424 + t21927 - t21930 + 0.36018386108879999999e-1 * t26773 + 24.0 * t28815 - 0.300153217574e-2 * t26783 - 0.300153217574e-2 * t26788;
    (t33653, t33662)
}
