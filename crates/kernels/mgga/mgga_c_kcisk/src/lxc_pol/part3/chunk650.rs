//! MGGA_C_KCISK lxc pol — lxc_pol part 3 (v3rho3_0) CSE chunk 650/938 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part3_v3rho3_0_chunk650<F: Float>(t10924: F, t608: F, t1724: F, t4859: F, t4910: F, t620: F, t342: F, t569: F, t969: F) -> (F, F, F, F) {
    let t10925 = t608 * t10924;
    let t10926 = t4859 * t1724;
    let t10928 = 1.0 / t4910 / t620;
    let t10929 = t10926 * t10928;
    let t10933 = t342 * t969 * t569;
    (t10925, t10926, t10929, t10933)
}
