//! MGGA_C_KCISK lxc pol — lxc_pol part 25 (v4rho3sigma_5) CSE chunk 359/1395 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part25_v4rho3sigma_5_chunk359<F: Float>(t2021: F, t2023: F, t1586: F, t2005: F, t2011: F, t2013: F, t2016: F, t782: F, t788: F) -> (F, F, F) {
    let t2024 = t2021 * t2023;
    let t2025 = t1586 * t2024;
    let t2028 = 0.2698618307426597582e-1 * t2005 * t788 + t2011 + 0.89953943580886586067e-2 * t2013 * t2016 - 0.2698618307426597582e-1 * t782 * t2025;
    (t2024, t2025, t2028)
}
