//! MGGA_C_KCISK lxc pol — lxc_pol part 28 (v4rho3sigma_8) CSE chunk 357/1456 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part28_v4rho3sigma_8_chunk357<F: Float>(t2028: F, t2029: F, t1795: F, t1804: F, t1866: F, t1897: F, t1902: F, t1990: F, t1994: F, t795: F, t801: F) -> (F, F, F, F, F) {
    let t2030 = t2028 * t2029;
    let t2033 = 0.11607361111111111111e-2 * t1795;
    let t2038 = t1990 * t795 - 0.193e0 * t1994 * t2030 + t2033 + 0.11607361111111111111e-2 * t1804 + 0.17411041666666666666e-2 * t1866 - 0.17411041666666666666e-2 * t1897 + 0.11607361111111111111e-2 * t1902;
    let t2040 = t801 * t801;
    let t2041 = 1.0 / t2040;
    (t2030, t2033, t2038, t2040, t2041)
}
