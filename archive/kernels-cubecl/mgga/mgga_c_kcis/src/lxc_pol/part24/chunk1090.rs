//! MGGA_C_KCIS lxc pol — lxc_pol part 24 (v4rho3sigma_6) CSE chunk 1090/1322 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part24_v4rho3sigma_6_chunk1090<F: Float>(t4546: F, t6272: F, t7718: F, t1020: F, t2179: F, t6481: F, t303: F, t26760: F, t6620: F, t1662: F, t4781: F, t4947: F) -> (F, F, F, F, F, F, F, F, F) {
    let t28915 = t4546 * t6272;
    let t28916 = t7718 * t28915;
    let t28917 = t1020 * t28916;
    let t28919 = t6481 * t2179;
    let t28920 = t303 * t28919;
    let t28924 = t26760 * t6620;
    let t28925 = t1020 * t28924;
    let t28927 = t4781 * t1662;
    let t28928 = t4947 * t28927;
    (t28915, t28916, t28917, t28919, t28920, t28924, t28925, t28927, t28928)
}
