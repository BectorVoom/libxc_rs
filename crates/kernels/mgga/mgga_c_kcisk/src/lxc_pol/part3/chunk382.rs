//! MGGA_C_KCISK lxc pol — lxc_pol part 3 (v3rho3_0) CSE chunk 382/938 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part3_v3rho3_0_chunk382<F: Float>(t2021: F, t2023: F, t1586: F, t2005: F, t2011: F, t2013: F, t2016: F, t782: F, t788: F, t791: F, t1795: F, t1804: F, t1866: F, t1897: F, t1902: F, t1990: F, t1994: F, t795: F) -> (F, F, F, F, F, F) {
    let t2024 = t2021 * t2023;
    let t2025 = t1586 * t2024;
    let t2028 = 0.2698618307426597582e-1 * t2005 * t788 + t2011 + 0.89953943580886586067e-2 * t2013 * t2016 - 0.2698618307426597582e-1 * t782 * t2025;
    let t2029 = 1.0 / t791;
    let t2030 = t2028 * t2029;
    let t2033 = 0.11607361111111111111e-2 * t1795;
    let t2038 = t1990 * t795 - 0.193e0 * t1994 * t2030 + t2033 + 0.11607361111111111111e-2 * t1804 + 0.17411041666666666666e-2 * t1866 - 0.17411041666666666666e-2 * t1897 + 0.11607361111111111111e-2 * t1902;
    (t2024, t2025, t2028, t2029, t2030, t2038)
}
